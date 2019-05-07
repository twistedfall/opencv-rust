# coding: utf-8
import logging
import os.path
import re
import shutil
import sys
import textwrap
from collections import OrderedDict
from itertools import chain
from pprint import pformat
from string import Template

# fixme returning MatAllocator (trait) by reference is bad, check knearestneighbour

# fixme add automatic lifetimes where needed
# fixme add getcpufeaturesline

if sys.version_info[0] >= 3:
    from io import StringIO
else:
    from cStringIO import StringIO


def template(text):
    """
    :type text: str
    :rtype: Template
    """
    if len(text) > 0 and text[0] == "\n":
        text = text[1:]
    return Template(textwrap.dedent(text))


def indent(text, level=1, chars_per_level=4, char=" "):
    """
    :type text: str
    :type level: int
    :type chars_per_level: int
    :type char: str
    :rtype: str
    """
    padding = char * (level * chars_per_level)
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

#
#       EXCEPTIONS TO AUTO GENERATION
#


# decls to inject before doing header parsing
decls_manual_pre = {
    "core": [
        ("class cv.Range", "", ["/Ghost"], []),
        ("class cv.Mat", "", ["/Ghost"], []),
        ("class cv.Algorithm", "", ["/Ghost"], []),
        ("class cv.DMatch", "", ["/Ghost", "/Simple"], []),
        ("class cv.KeyPoint", "", ["/Ghost", "/Simple"], []),
        ("class cv.RotatedRect", "", ["/Ghost"], []),
        ("class cv.TermCriteria", "", ["/Ghost"], []),
    ]
}

# decls to inject after doing header parsing
decls_manual_post = {
    "core": (
        ["cv.Mat.size", "Size", ["/C"], []],
        # ["cv.Mat.step", "size_t", ["/C"], []],
    )
}

renamed_funcs = {  # todo check if any "new" is required
    # calib3D
    # "cv_StereoBM_StereoBM": "default",
    # "cv_StereoBM_StereoBM_int_preset_int_ndisparities_int_SADWindowSize": "new",
    # "cv_StereoSGBM_StereoSGBM": "default",
    # "cv_StereoSGBM_StereoSGBM_int_minDisparity_int_numDisparities_int_SADWindowSize_int_P1_int_P2_int_disp12MaxDiff_int_preFilterCap_int_uniquenessRatio_int_speckleWindowSize_int_speckleRange_bool_fullDP": "new",
    "cv_findEssentialMat_Mat_points1_Mat_points2_Mat_cameraMatrix_int_method_double_prob_double_threshold_Mat_mask": "find_essential_map_matrix",
    "cv_findHomography_Mat_srcPoints_Mat_dstPoints_int_method_double_ransacReprojThreshold_Mat_mask_int_maxIters_double_confidence": "find_homography_full",
    "cv_fisheye_projectPoints_Mat_objectPoints_Mat_imagePoints_Mat_rvec_Mat_tvec_Mat_K_Mat_D_double_alpha_Mat_jacobian": "fisheye_project_points",
    "cv_fisheye_undistortImage_Mat_distorted_Mat_undistorted_Mat_K_Mat_D_Mat_Knew_Size_new_size": "fisheye_undistort_image",
    "cv_fisheye_undistortPoints_Mat_distorted_Mat_undistorted_Mat_K_Mat_D_Mat_R_Mat_P": "fisheye_undistort_points",
    "cv_recoverPose_Mat_E_Mat_points1_Mat_points2_Mat_cameraMatrix_Mat_R_Mat_t_Mat_mask": "recover_pose_matrix",
    # core
    "cvAlloc_size_t_size": "-",
    "cvClone_const_void_X_struct_ptr": "-",
    "cvFree__void_X_ptr": "-",
    "cv_addImpl_int_flag_const_char_X_func": "-",
    # "cv_Algorithm_set_String_name_Mat_value": "set_mat",
    # "cv_Algorithm_set_String_name_VectorOfMat_value": "set_VectorOfMat",
    # "cv_Algorithm_set_String_name_bool_value": "set_bool",
    # "cv_Algorithm_set_String_name_double_value": "set_double",
    # "cv_Algorithm_set_String_name_int_value": "set_int",
    # "cv_Algorithm_set_String_name_String_value": "set_string",
    # "cv_Algorithm_setInt_String_name_int_value" : "-",
    # "cv_Algorithm_setDouble_String_name_double_value" : "-",
    # "cv_Algorithm_setBool_String_name_bool_value" : "-",
    # "cv_Algorithm_setString_String_name_String_value" : "-",
    # "cv_Algorithm_setMat_String_name_Mat_value" : "-",
    # "cv_Algorithm_setMatVector_String_name_VectorOfMat_value" : "-",
    "cv_MatExpr_type_const": "typ",
    "cv_Mat_Mat": "new",
    "cv_Mat_Mat_Mat_m": "copy",
    "cv_Mat_Mat_Mat_m_const_Range_ranges": "ranges",
    "cv_Mat_Mat_Mat_m_Range_rowRange_Range_colRange": "rowscols",
    "cv_Mat_Mat_Mat_m_Rect_roi": "roi",
    "cv_Mat_Mat_Size_size_int_type": "new_size",
    "cv_Mat_Mat_Size_size_int_type_Scalar_s": "new_size_with_default",
    "cv_Mat_Mat_int_rows_int_cols_int_type": "new_rows_cols",
    "cv_Mat_Mat_int_rows_int_cols_int_type_Scalar_s": "new_rows_cols_with_default",
    "cv_Mat_colRange_const_Range_r": "colrange",
    "cv_Mat_colRange_const_int_startcol_int_endcol": "colbounds",
    "cv_Mat_copyTo_const_Mat_m_Mat_mask": "copy_to_masked",
    "cv_Mat_create_Size_size_int_type": "-",
    "cv_Mat_create_int_rows_int_cols_int_type": "-",
    "cv_Mat_diag_Mat_d": "diag_new_mat",
    "cv_Mat_diag_const_int_d": "diag",
    "cv_Mat_ptr_int_i0": "ptr_mut",
    "cv_Mat_ptr_const_int_i0": "ptr",
    "cv_Mat_ptr_int_row_int_col": "ptr_2d_mut",
    "cv_Mat_ptr_const_int_row_int_col": "ptr_2d",
    "cv_Mat_ptr_int_i0_int_i1_int_i2": "ptr_3d_mut",
    "cv_Mat_ptr_const_int_i0_int_i1_int_i2": "ptr_3d",
    "cv_Mat_at_int_i0": "at_mut",
    "cv_Mat_at_int_row_int_col": "at_2d_mut",
    "cv_Mat_at_const_int_row_int_col": "at_2d",
    "cv_Mat_at_int_i0_int_i1_int_i2": "at_3d_mut",
    "cv_Mat_at_const_int_i0_int_i1_int_i2": "at_3d",
    "cv_Mat_resize_size_t_sz": "resize",
    "cv_Mat_resize_size_t_sz_Scalar_s": "resize_with_default",
    "cv_Mat_rowRange_const_Range_r": "row_range",
    "cv_Mat_rowRange_const_int_startrow_int_endrow": "rowbounds",
    "cv_Mat_type_const": "typ",
    "cv_PCA_PCA": "default",
    "cv_PCA_PCA_Mat_data_Mat_mean_int_flags_double_retainedVariance": "new_mat_variance",
    "cv_PCA_PCA_Mat_data_Mat_mean_int_flags_int_maxComponents": "new_mat_max",
    "cv_PCA_backProject_const_Mat_vec_Mat_result": "back_project_to",
    "cv_PCA_project_const_Mat_vec_Mat_result": "project_to",
    "cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_double_retainedVariance": "pca_compute_variance",
    "cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_int_maxComponents": "pca_compute",
    "cv_Range_Range": "default",
    "cv_Range_Range_int__start_int__end": "new",
    "cv_RotatedRect_RotatedRect": "default",
    "cv_RotatedRect_RotatedRect_Point2f_center_Size2f_size_float_angle": "new",
    "cv_RotatedRect_RotatedRect_Point2f_point1_Point2f_point2_Point2f_point3": "for_points",
    "cv_TermCriteria_TermCriteria": "default",
    "cv_TermCriteria_TermCriteria_int_type_int_maxCount_double_epsilon": "new",
    "cv_UMat_type_const": "typ",
    "cv_UMat_copyTo_const_Mat_m": "copy_to",
    "cv_UMat_copyTo_const_Mat_m_Mat_mask": "copy_to_masked",
    "cv_calcCovarMatrix_Mat_samples_Mat_covar_Mat_mean_int_flags_int_ctype": "calc_covar_matrix_arrays",
    "cv_calcCovarMatrix_Mat_samples_int_nsamples_Mat_covar_Mat_mean_int_flags_int_ctype": "calc_covar_matrix",
    "cv_clipLine_Size_imgSize_Point_pt1_Point_pt2": "clip_line_size",
    "cv_clipLine_Size2l_imgSize_Point2l_pt1_Point2l_pt2": "clip_line_size_i64",
    "cv_clipLine_Rect_imgRect_Point_pt1_Point_pt2": "clip_line",
    "cv_cv_abs_short_x": "-",
    "cv_cv_abs_uchar_x": "-",
    "cv_divide_double_scale_Mat_src2_Mat_dst_int_dtype": "divide",
    "cv_divide_Mat_src1_Mat_src2_Mat_dst_double_scale_int_dtype": "divide_mat",
    "cv_ellipse_Mat_img_Point_center_Size_axes_double_angle_double_startAngle_double_endAngle_Scalar_color_int_thickness_int_lineType_int_shift": "ellipse",
    "cv_ellipse_Mat_img_RotatedRect_box_Scalar_color_int_thickness_int_lineType": "ellipse_new_rotated_rect",
    "cv_ellipse2Poly_Point2d_center_Size2d_axes_int_angle_int_arcStart_int_arcEnd_int_delta_VectorOfPoint2d_pts": "ellipse2_poly_f64",
    # "cv_eigen_Mat_src_Mat_eigenvalues_int_lowindex_int_highindex": "eigen_indexes", ?
    # "cv_eigen_Mat_src_bool_computeEigenvectors_Mat_eigenvalues_Mat_eigenvectors": "eigen", ?
    # "cv_eigen_Mat_src_Mat_eigenvalues_Mat_eigenvectors_int_lowindex_int_highindex": "eigen_vectors", ?
    "cv_hconcat_Mat_src_size_t_nsrc_Mat_dst": "-",
    "cv_max_Mat_src1_Mat_src2_Mat_dst": "max_mat_mat",
    "cv_merge_Mat_mv_size_t_count_Mat_dst": "-",
    "cv_min_Mat_src1_Mat_src2_Mat_dst": "min_mat_mat",
    "cv_norm_Mat_src1_Mat_src2_int_normType_Mat_mask": "norm_with_type",
    "cv_norm_Mat_src1_int_normType_Mat_mask": "norm",
    "cv_rectangle_Mat_img_Point_pt1_Point_pt2_Scalar_color_int_thickness_int_lineType_int_shift": "rectangle_points",
    "cv_rectangle_Mat_img_Rect_rec_Scalar_color_int_thickness_int_lineType_int_shift": "rectangle",
    "cv_repeat_Mat_src_int_ny_int_nx_Mat_dst": "repeat_to",
    "cv_repeat_Mat_src_int_ny_int_nx": "repeat",
    "cv_split_Mat_m_VectorOfMat_mv": "split",
    "cv_split_Mat_src_Mat_mvbegin": "split_at",
    "cv_vconcat_Mat_src_size_t_nsrc_Mat_dst": "-",
    "cv_getNumberOfCPUs": "get_number_of_cpus",
    # features2d
    "cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression": "AGAST",
    "cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression_int_type": "AGAST_with_type",
    "cv_BOWKMeansTrainer_cluster_const": "default",
    "cv_BOWKMeansTrainer_cluster_const_Mat_descriptors": "new",
    "cv_BOWKMeansTrainer_BOWKMeansTrainer_int_clusterCount_TermCriteria_termcrit_int_attempts_int_flags": "new_with_criteria",
    "cv_BOWImgDescriptorExtractor_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_imgDescriptor_VectorOfVectorOfint_pointIdxsOfClusters_Mat_descriptors": "compute_desc",
    "cv_DMatch_DMatch": "default",
    "cv_DMatch_DMatch_int__queryIdx_int__trainIdx_float__distance": "new",
    "cv_DMatch_DMatch_int__queryIdx_int__trainIdx_int__imgIdx_float__distance": "new_index",
    # "cv_DescriptorExtractor_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_descriptors": "compute",
    # "cv_DescriptorExtractor_compute_VectorOfMat_images_VectorOfVectorOfKeyPoint_keypoints_VectorOfMat_descriptors": "compute_n",
    "cv_DescriptorMatcher_knnMatch_const_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_int_k_Mat_mask_bool_compactResult": "knn_train_matches",
    "cv_DescriptorMatcher_knnMatch_Mat_queryDescriptors_VectorOfVectorOfDMatch_matches_int_k_VectorOfMat_masks_bool_compactResult": "knn_matches",
    "cv_DescriptorMatcher_match_Mat_queryDescriptors_VectorOfDMatch_matches_VectorOfMat_masks": "matches",
    "cv_DescriptorMatcher_match_const_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfDMatch_matches_Mat_mask": "train_matches",
    "cv_DescriptorMatcher_radiusMatch_const_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_float_maxDistance_Mat_mask_bool_compactResult": "train_radius_matches",
    "cv_DescriptorMatcher_radiusMatch_Mat_queryDescriptors_VectorOfVectorOfDMatch_matches_float_maxDistance_VectorOfMat_masks_bool_compactResult": "radius_matches",
    # "cv_FREAK_FREAK_FREAK_rhs": "copy",
    # "cv_FREAK_FREAK_bool_orientationNormalized_bool_scaleNormalized_float_patternScale_int_nOctaves_VectorOfint_selectedPairs": "new",
    "cv_FAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression": "FAST",
    "cv_FAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression_int_type": "FAST_with_type",
    "cv_Feature2D_detect_VectorOfMat_images_VectorOfVectorOfKeyPoint_keypoints_VectorOfMat_masks": "detect_n",
    "cv_KeyPoint_KeyPoint": "default",
    "cv_KeyPoint_KeyPoint_Point2f__pt_float__size_float__angle_float__response_int__octave_int__class_id": "new_point",
    "cv_KeyPoint_KeyPoint_float_x_float_y_float__size_float__angle_float__response_int__octave_int__class_id": "new_coords",
    "cv_KeyPoint_convert_VectorOfKeyPoint_keypoints_VectorOfPoint2f_points2f_VectorOfint_keypointIndexes": "convert_from",
    "cv_KeyPoint_convert_VectorOfPoint2f_points2f_VectorOfKeyPoint_keypoints_float_size_float_response_int_octave_int_class_id": "convert_to",
    "cv_MatStep_MatStep": "default",
    "cv_MatStep_MatStep_size_t_s": "new",
    "cv_drawMatches_Mat_img1_VectorOfKeyPoint_keypoints1_Mat_img2_VectorOfKeyPoint_keypoints2_VectorOfDMatch_matches1to2_Mat_outImg_Scalar_matchColor_Scalar_singlePointColor_VectorOfchar_matchesMask_int_flags": "draw_matches",
    "cv_drawMatches_Mat_img1_VectorOfKeyPoint_keypoints1_Mat_img2_VectorOfKeyPoint_keypoints2_VectorOfVectorOfDMatch_matches1to2_Mat_outImg_Scalar_matchColor_Scalar_singlePointColor_VectorOfVectorOfchar_matchesMask_int_flags": "draw_vector_matches",
    "cv_setImpl_int_flags": "-",
    "cv_setUseCollection_bool_flag": "-",
    "cv_useCollection": "-",
    # imgcodecs
    "cv_imdecode_Mat_buf_int_flags": "decode",
    "cv_imdecode_Mat_buf_int_flags_Mat_dst": "decode_to",  # fixme, make sure dst is &mut
    # imgproc
    "cv_Canny_Mat_dx_Mat_dy_Mat_edges_double_threshold1_double_threshold2_bool_L2gradient": "canny_derivative",
    # "cv_GeneralizedHough_detect_Mat_image_Mat_positions_Mat_votes_int_cannyThreshold": "detect",
    # "cv_GeneralizedHough_detect_Mat_edges_Mat_dx_Mat_dy_Mat_positions_Mat_votes": "detect_edges",
    # "cv_GeneralizedHough_setTemplate_Mat_edges_Mat_dx_Mat_dy_Point_templCenter": "set_template_edges",
    # "cv_GeneralizedHough_setTemplate_Mat_templ_int_cannyThreshold_Point_templCenter": "set_template_templ",
    "cv_Moments_Moments": "default",
    "cv_Moments_Moments_double_m00_double_m10_double_m01_double_m20_double_m11_double_m02_double_m30_double_m21_double_m12_double_m03": "new",
    "cv_Subdiv2D_Subdiv2D": "default",
    "cv_Subdiv2D_Subdiv2D_Rect_rect": "new",
    "cv_Subdiv2D_insert_Point2f_pt": "insert",
    "cv_Subdiv2D_insert_VectorOfPoint2f_ptvec": "insert_n",
    "cv_findContours_Mat_image_VectorOfMat_contours_Mat_hierarchy_int_mode_int_method_Point_offset": "find_contours_with_hierarchy",
    "cv_distanceTransform_Mat_src_Mat_dst_Mat_labels_int_distanceType_int_maskSize_int_labelType": "distance_transform_labels",
    "cv_distanceTransform_Mat_src_Mat_dst_int_distanceType_int_maskSize": "distance_transform",
    "cv_integral_Mat_src_Mat_sum_Mat_sqsum_Mat_tilted_int_sdepth_int_sqdepth": "integral_titled_sq",
    "cv_integral_Mat_src_Mat_sum_Mat_sqsum_Mat_tilted_int_sdepth": "integral_tilted",
    "cv_integral_Mat_src_Mat_sum_Mat_sqsum_int_sdepth_int_sqdepth": "integral_sq_depth",
    "cv_integral_Mat_src_Mat_sum_Mat_sqsum_int_sdepth": "integral_sq",
    "cv_integral_Mat_src_Mat_sum_int_sdepth": "integral",
    "cv_hal_resize_int_src_type_const_uchar_X_src_data_size_t_src_step_int_src_width_int_src_height_uchar_X_dst_data_size_t_dst_step_int_dst_width_int_dst_height_double_inv_scale_x_double_inv_scale_y_int_interpolation": "hal_resize",
    # ml
    "cv_ml_ParamGrid_ParamGrid_double__minVal_double__maxVal_double__logStep": "for_range",
    # objdetect": "",
    "cv_CascadeClassifier_CascadeClassifier": "default",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels": "detect_multi_scale_levels",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize": "detect_multi_scale_num",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize": "detect_multi_scale",
    "cv_HOGDescriptor_HOGDescriptor_HOGDescriptor_d": "copy",
    "cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_VectorOfdouble_foundWeights_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping": "detect_multi_scale",
    "cv_HOGDescriptor_detectMultiScale_const_Mat_img_VectorOfRect_foundLocations_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping": "detect_multi_scale_weights",
    "cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_VectorOfdouble_weights_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations": "detect_weights",
    "cv_HOGDescriptor_detect_const_Mat_img_VectorOfPoint_foundLocations_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations": "detect",
    "cv_groupRectangles_VectorOfRect_rectList_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_int_groupThreshold_double_eps": "group_rectangles_weights_rejects",
    "cv_groupRectangles_VectorOfRect_rectList_VectorOfint_weights_int_groupThreshold_double_eps": "group_rectangle_weights",
    "cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps": "group_rectangle",
    "cv_groupRectangles_VectorOfRect_rectList_int_groupThreshold_double_eps_VectorOfint_weights_VectorOfdouble_levelWeights": "group_rectangle_levelweights",
    # "cv_linemod_ColorGradient_ColorGradient": "default",
    # "cv_linemod_ColorGradient_ColorGradient_float_weak_threshold_size_t_num_features_float_strong_threshold": "new",
    # "cv_linemod_DepthNormal_DepthNormal": "default",
    # "cv_linemod_DepthNormal_DepthNormal_int_distance_threshold_int_difference_threshold_size_t_num_features_int_extract_threshold": "new",
    # "cv_linemod_Feature_Feature": "default",
    # "cv_linemod_Feature_Feature_int__x_int__y_int__label": "-",
    # "cv_linemod_Feature_Feature_int_x_int_y_int_label": "new",
    # photo
    "cv_fastNlMeansDenoisingColored_Mat_src_Mat_dst_float_h_float_hColor_int_templateWindowSize_int_searchWindowSize": "fast_nl_means_denoising_color",
    "cv_fastNlMeansDenoising_Mat_src_Mat_dst_VectorOffloat_h_int_templateWindowSize_int_searchWindowSize_int_normType": "fast_nl_means_denoising_vec",
    "cv_fastNlMeansDenoising_Mat_src_Mat_dst_float_h_int_templateWindowSize_int_searchWindowSize": "fast_nl_means_denoising_window",
    "cv_AlignMTB_process_VectorOfMat_src_VectorOfMat_dst_Mat_times_Mat_response": "process_with_response",
    "cv_MergeDebevec_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response": "process_with_response",
    "cv_MergeMertens_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response": "process_with_response",
    "cv_MergeRobertson_process_VectorOfMat_src_Mat_dst_Mat_times_Mat_response": "process_with_response",
    # video
    # "cv_BackgroundSubtractorMOG2_BackgroundSubtractorMOG2": "default",
    # "cv_BackgroundSubtractorMOG2_BackgroundSubtractorMOG2_int_history_float_varThreshold_bool_bShadowDetection": "new",
    # "cv_BackgroundSubtractorMOG_BackgroundSubtractorMOG": "default",
    # "cv_BackgroundSubtractorMOG_BackgroundSubtractorMOG_int_history_int_nmixtures_double_backgroundRatio_double_noiseSigma": "new",
    "cv_KalmanFilter_KalmanFilter": "default",
    "cv_KalmanFilter_KalmanFilter_int_dynamParams_int_measureParams_int_controlParams_int_type": "new",
    # "cv_calcOpticalFlowSF_Mat_from_Mat_to_Mat_flow_int_layers_int_averaging_block_size_int_max_flow": "new",
    # "cv_calcOpticalFlowSF_Mat_from_Mat_to_Mat_flow_int_layers_int_averaging_block_size_int_max_flow_double_sigma_dist_double_sigma_color_int_postprocess_window_double_sigma_dist_fix_double_sigma_color_fix_double_occ_thr_int_upscale_averaging_radius_double_upscale_sigma_dist_double_upscale_sigma_color_double_speed_up_thr": "new_sigmas",
    # videostab
    # "cv_videostab_GaussianMotionFilter_GaussianMotionFilter_int__radius_float__stdev" : "-",
    # videoio
    "cv_VideoCapture_VideoCapture": "default",
    "cv_VideoCapture_VideoCapture_int_index": "index",
    "cv_VideoCapture_VideoCapture_String_filename": "filename",
    "cv_VideoCapture_VideoCapture_String_filename_int_apiPreference": "filename_api",
    "cv_VideoCapture_open_int_index": "open_index",
    "cv_VideoCapture_open_String_filename": "open_filename",
    "cv_VideoCapture_open_String_filename_int_apiPreference": "open_filename_api",
    "cv_VideoWriter_VideoWriter": "default",
    # utility
    "cv_getImpl_VectorOfint_impl_VectorOfString_funName": "-",
    # dnn
    "cv_dnn_<unnamed>_is_neg_int_i": "-",
    "cv_dnn_NMSBoxes_VectorOfRotatedRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k": "nms_boxes_rotated",
    "cv_dnn_NMSBoxes_VectorOfRect2d_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k": "nms_boxes_rotated_f64",
    "cv_dnn_Dict_ptr_String_key": "ptr_mut",
    "cv_dnn_Dict_ptr_const_String_key": "ptr",
}

_templ_const = template("""
// identifier: ${identifier}
${doc_comment}${visibility}fn ${r_name}<T: core::ValidMatElement>(${args}) -> Result<&T> { ${pre_call_args}self._${r_name}(${forward_args}) }
            
""")
_templ_mut = template("""
// identifier: ${identifier}
${doc_comment}${visibility}fn ${r_name}<T: core::ValidMatElement>(${args}) -> Result<&mut T> { ${pre_call_args}self._${r_name}(${forward_args}) }

""")

func_manual_implementation = {
    "cv_Mat_at_int_i0": {
        "rust_safe": _templ_mut,
    },
    "cv_Mat_at_const_int_i0": {
        "rust_safe": _templ_const,
    },
    "cv_Mat_at_int_i0_int_i1_int_i2": {
        "rust_safe": _templ_mut,
    },
    "cv_Mat_at_const_int_i0_int_i1_int_i2":  {
        "rust_safe": _templ_const,
    },
    "cv_Mat_at_int_row_int_col": {
        "rust_safe": _templ_mut,
    },
    "cv_Mat_at_const_int_row_int_col": {
        "rust_safe": _templ_const,
    },
}

class_ignore_list = (
    # core
    "Cv[A-Z]",
    "cv::Mat::MStep",
    "cv::Mat::MSize",
    "Ipl.*",
    "BinaryFunc", "ConvertData", "ConvertScaleData",
    "cv::Exception", "cv::ErrorCallback",
    "cv::RNG.*", # maybe
    "cv::SVD",
    "cv::hal",
    "cv::MatAllocator",
    "cv::SparseMat",  # fixme
    "cv::TLSDataContainer",
    "NAryMatIterator",
    "cv::MatConstIterator",
    "cv::_InputArray", "cv::_OutputArray", "cv::_InputOutputArray",
    # stitching
    "cv::CylindricalWarperGpu", "cv::PlaneWarperGpu", "cv::SphericalWarperGpu",
    # videostab
    "cv::videostab::DensePyrLkOptFlowEstimatorGpu",
    "cv::videostab::KeypointBasedMotionEstimatorGpu",
    "cv::videostab::MoreAccurateMotionWobbleSuppressorGpu",
    "cv::videostab::SparsePyrLkOptFlowEstimatorGpu",
    # ml
    "cv::ml::SimulatedAnnealingSolverSystem",  # only defined in docs
    # dnn
    "cv::dnn::_Range",
)

type_replace = {
    "unsigned": "uint",
    "InputArray": "cv::Mat",
    "InputArrayOfArrays": "vector<cv::Mat>",
    "OutputArray": "cv::Mat",
    "OutputArrayOfArrays": "vector<cv::Mat>",
    "InputOutputArrayOfArrays": "vector<cv::Mat>",
    "InputOutputArray": "cv::Mat",
    "_InputArray": "cv::Mat",
    "_OutputArray": "cv::Mat",
    "_InputOutputArray": "cv::Mat",
    "_Range": "cv::Range",
    "Point_<int>": "Point2i",
    "Point_<int64>": "Point2l",
    "Point_<float>": "Point2f",
    "Point_<double>": "Point2d",
    "Rect_<int>": "Rect2i",
    "Rect_<float>": "Rect2f",
    "Rect_<double>": "Rect2d",
    "Size_<int64>": "Size2l",
    "Size_<float>": "Size2f",
    "Size_<double>": "Size2d",
    "Scalar_<double>": "Scalar",
}

func_ignore_list = (
    "cv.glob", "cv.fastFree", "cv.fastMalloc",
    "cv.getBuildInformation", "cv.scalarToRawData", "cv::noArray", "()", "cv.Mat.MSize.operator[]",
    "const int*", "=", "==", "!=", "--", "++", "*", ">>", "<<", "<", ">", "operator==", "operator()",
    "cv.Mat.MStep.operator[]",
    "cv.abs", "cvCeil", "cvFloor", "cvIsInf", "cvIsNaN", "cvRound",
    "cv.swap",
    "cv.minMaxLoc", "cv.minMaxIdx", # return prims by pointer
    "cv.merge", # pointer to array of matrix
    "cv.split",
    "cv.mixChannels", "cv.insertChannel",
    "cv.hconcat", "cv.vconcat", "cv.repeat", # maybe: repeat(*((const Mat&*)src), ny, nx)
    "cv.min", "cv.max", "cv.exp", "cv.log", "cv.fastAtan2", # return prims by pointer (may be make to work)
    "cv.magnitude", "cv.patchNaNs", "cv.setIdentity", "cv.completeSymm", "cv.calcCovarMatrix",
    "cv.fillConvexPoly", "cv.fillPoly", "cv.polylines", # Point**
    "cv.getTextSize", # return prim by ptr
    "cv.SparseMat.Hdr.clear",
    "cv.PCA.computeVar", # what ?
    "cvSetPreprocessFuncWin32_", "cvSetPostprocessFuncWin32_",
    # features
    "cv.BOWImgDescriptorExtractor.getVocabulary",
)

# regular expressions to ignore matching constant names
const_ignore_list = (
    "CV_EXPORTS_W", "CV_MAKE_TYPE",
    "CV_IS_CONT_MAT", "CV_RNG_COEFF", "IPL_IMAGE_MAGIC_VAL",
    "CV_SET_ELEM_FREE_FLAG", "CV_FOURCC_DEFAULT",
    "CV_WHOLE_ARR", "CV_WHOLE_SEQ", "CV_PI", "CV_2PI", "CV_LOG2",
    "CV_TYPE_NAME_IMAGE",
    "CV_SUPPRESS_DEPRECATED_START",
    "CV_SUPPRESS_DEPRECATED_END",
    "__CV_BEGIN__", "__CV_END__", "__CV_EXIT__",
    "CV_IMPL_IPP", "CV_IMPL_MT", "CV_IMPL_OCL", "CV_IMPL_PLAIN",
    "CV_TRY", "CV_CATCH_ALL",
    "CV__DEBUG_NS_",
    "UINT64_1",
    "CV_STRUCT_INITIALIZER", "CV__ENABLE_C_API_CTORS",
    "VSX_IMPL_MULH_",
    "CV__DNN_EXPERIMENTAL_NS_",
    "CV_Sts",
)

#
#       TYPES MAPPING
#

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
    "unsigned int": {"cpp_extern": "unsigned int", "rust_local": "u32"},
    "uint32_t": {"cpp_extern": "uint32_t", "rust_local": "u32"},

    "size_t": {"cpp_extern": "std::size_t", "rust_local": "size_t"},

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

type_manual_declaration = {}


def _base_type_alias(module, rust_name, rust_definition, cpp_field_type, cpp_fields):
    if module not in type_manual_declaration:
        type_manual_declaration[module] = {}
    type_manual_declaration[module][rust_name] = {
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
            ("s", "short"),\
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
        if t[0] != "l":
            _base_type_alias("core", "Vec{}{}".format(s, t[0]), "core::Vec{}<{}>".format(s, rust_local), t[1], ("data[{}]".format(s),))

forced_trait_classes = ("cv::Algorithm", "cv::BackgroundSubtractor", "cv::dnn::Layer")

boxed_type_fields = {
    "RotatedRect": {
        "center": "Point2f",
        "size": "Size2f",
        "angle": "float",
    }
}

reserved_rename = {
    "type": "_type",
    "box": "_box",
    "ref": "_ref",
    "in": "_in",
    "use": "_use",
}

static_modules = ("core", "sys", "types")

#
#       TEMPLATES
#

T_CPP_MODULE = template("""
//
// This file is auto-generated, please don't edit!
//

#include "stdint.h"
#include "common.h"

typedef int64_t int64;

#include <iostream>
#include "opencv2/opencv_modules.hpp"
#include <string>
#include "common_opencv.h"

using namespace cv;
#include "types.h"
$includes


extern "C" {

#include "return_types.h"

$code

} // extern "C"

""")

const_private_list = (
    "CV_MOP_.+",
    "CV_INTER_.+",
    "CV_THRESH_.+",
    "CV_INPAINT_.+",
    "CV_RETR_.+",
    "CV_CHAIN_APPROX_.+",
    "OPPONENTEXTRACTOR",
    "GRIDDETECTOR",
    "PYRAMIDDETECTOR",
    "DYNAMICDETECTOR",
)


#
#       Helpers
#

def camel_case_to_snake_case(name):
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def bump_counter(name):
    """
    :type name: str
    :rtype: str
    """
    pos = len(name) - 1
    for pos in xrange(len(name) - 1, 0, -1):
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
        :type namespaces: frozenset
        """
        self.gen = gen
        self.fullname, self.namespace, self.classpath, self.classname, self.name = self.do_parse_name(name, namespaces)
        logging.info(
            "parse_name: %s with %s -> fullname:%s namespace:%s classpath:%s classname:%s name:%s" %
            (name, namespaces, self.fullname, self.namespace, self.classpath, self.classname, self.name)
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
        typ = arg_tuple[0]
        self.type = self.gen.get_type_info(typ)
        self.name = arg_tuple[1]
        if not self.name:
            self.name = "unnamed_arg"
        self.rsname = camel_case_to_snake_case(reserved_rename.get(self.name, self.name))
        self.defval = ""
        if len(arg_tuple) > 2:
            self.defval = arg_tuple[2]
        self.out = ""
        if typ in ("OutputArray", "OutputArrayOfArrays") or len(arg_tuple) > 3 and "/O" in arg_tuple[3]:
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
    KIND_CONSTRUCTOR = "(constructor)"

    TEMPLATES = {
        "cpp": template("""
                // ${identifier}
                // parsed: ${fullname}
                // as:     ${repr}
                ${args}// Return value: ${rv_type}
                ${return_wrapper_type} ${c_name}(${decl_cpp_args}) {
                    try {
                ${code}
                    } CVRS_CATCH(${return_wrapper_type})
                }

            """),

        "cpp_doc_arg": template("""// Arg ${repr}${ptr} ${type} = ${defval}${ignored}\n"""),

        "rust_safe": template("""
                // identifier: ${identifier}
                ${doc_comment}${visibility}fn ${r_name}${generic_decl}(${args}) -> Result<$rv_rust_full> {${pre_call_args}
                    unsafe { sys::${c_name}(${call_args}) }.into_result()${rv}
                }
                
            """),

        "rust_safe_rv_string": template("""
                .map(crate::templ::receive_string)"""),

        "rust_safe_rv_string_mut": template("""
                .map(crate::templ::receive_string_mut)"""),

        "rust_safe_rv_const_raw_ptr": template("""
            .and_then(|x| unsafe { x.as_ref() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))"""),

        "rust_safe_rv_mut_raw_ptr": template("""
            .and_then(|x| unsafe { x.as_mut() }.ok_or_else(|| Error::new(core::StsNullPtr, format!("Function returned Null pointer"))))"""),

        "rust_safe_rv_vector_box_ptr": template(""".map(|x| ${rv_rust_full} { ptr: x })"""),

        "rust_safe_rv_other": template(""""""),

        "rust_externs": template("""
                #[doc(hidden)] pub fn ${c_name}(${args}) -> ${return_wrapper_type};
             """),
    }

    def __init__(self, gen, module, decl, namespaces=frozenset()):  # [ funcname, return_ctype, [modifiers], [args] ]
        """
        :type gen: RustWrapperGenerator
        :type module: str
        :type decl: list
        :type namespaces: frozenset
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.module = module

        self.is_ignored = False
        if self.classname and not self.classname.startswith("operator"):
            self.ci = gen.get_class(self.classname)
            if not self.ci:
                if self.classname == "std" or "<" in self.classname:
                    self.is_ignored = True
                else:
                    raise NameError("class not found: " + self.classname)
            if "/A" in decl[2]:
                self.ci.is_trait = True
            if self.classname == self.name:
                self.kind = self.KIND_CONSTRUCTOR
                self.name = "new"
                self.type = gen.get_type_info(self.classname)
            else:
                self.kind = self.KIND_METHOD
                self.type = gen.get_type_info(decl[1])
        else:
            self.kind = self.KIND_FUNCTION
            self.ci = None  # type: ClassInfo
            self.type = gen.get_type_info(decl[1])

        self.identifier = self.fullname.replace("::", "_")

        self.is_ignored = self.is_ignored or "/H" in decl[2] or "/I" in decl[2]

        self.is_const = "/C" in decl[2]
        self.is_static = "/S" in decl[2]
        self.fake_attrgetter = "/ATTRGETTER" in decl[2]
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
            self.identifier += "_" + ai.type.rust_safe_id + "_" + ai.name
            if isinstance(ai.type, CallbackTypeInfo):
                self.has_callback_arg = True

        if self.has_callback_arg and not has_userdata_arg:
            logging.info("ignore function with callback, but without userdata %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if len(decl) > 5:
            self.comment = decl[5].encode("ascii", "ignore")
        else:
            self.comment = ""

        self.struct_attrname = decl[6] if self.fake_attrgetter else None

        self.cname = self.cppname = self.name

        if self.name.startswith("~"):
            logging.info("ignore destructor %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if self.name.startswith("operator") or self.fullname.startswith("operator "):
            logging.info("ignore %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

    def _get_manual_implementation(self, section, template_vars):
        if self.identifier in func_manual_implementation:
            templ = func_manual_implementation[self.identifier]
            if section in templ:
                if templ[section] == "~":
                    return None
                return templ[section].substitute(template_vars)
            else:
                return ""
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

        if self.identifier in func_manual_implementation:
            return None

        if self.name.startswith("operator"):
            return "can not map %s yet"%(self.name)

        for f in func_ignore_list:
            if self.fullname.endswith(f):
                return "manual ignore from list"

        if renamed_funcs.get(self.identifier) == "-":
            return "ignored by renamed table"

        if not self.rv_type():
            return "rv_header_type returns None. this is an error. (class not found ?)"

        if self.type.is_ignored:
            return "return type class %s is ignored"%(self.type)

        if self.rv_type().is_ignored:
            return "return value type is ignored"

        if self.kind == self.KIND_CONSTRUCTOR and self.ci.is_trait:
            return "skip constructor of abstract class"

        for a in self.args:
            if a.type.is_ignored:
                return "can not map type %s yet"%(a.type)

        return None

    def c_name(self):
        # fixme identifier without cv_core_ prefix
        return "cv_%s_%s" % (self.module, self.identifier)

    def r_name(self):
        if renamed_funcs.get(self.identifier):
            return renamed_funcs[self.identifier]
        name = "new" if self.is_constructor() else self.name

        return camel_case_to_snake_case(reserved_rename.get(name, name))

    def gen_cpp(self):
        decl_cpp_args = []
        args = ""
        if self.is_instance_method():
            # fixme? add RawPtr handling
            decl_cpp_args.append(self.ci.type_info().cpp_extern + " instance")

        call_cpp_args = []
        for a in self.args:
            ignored = ptr = ""
            if a.type.is_ignored:
                ignored = " (ignored)"
            if isinstance(a.type, RawPtrTypeInfo):
                ptr = " (ptr)"
            args += FuncInfo.TEMPLATES["cpp_doc_arg"].substitute(combine_dicts(a.__dict__, {
                "repr": repr(a),
                "ptr": ptr,
                "ignored": ignored
            }))

            decl_cpp_args.append(a.type.cpp_arg_func_decl(a.name, a.is_output()))
            call_cpp_args.append(a.type.cpp_arg_func_call(a.name, a.is_output()))

        # cpp method call with prefix
        if self.is_constructor():
            call_name = self.ci.fullname
        elif self.ci is None or self.is_static:
            if self.namespace == "":
                call_name = self.cppname
            else:
                call_name = self.fullname
        elif self.fake_attrgetter:
            call_name = self.ci.type_info().cpp_method_call_name(self.struct_attrname)
        else:
            call_name = self.ci.type_info().cpp_method_call_name(self.cppname)

        # actual call
        if self.fake_attrgetter:
            code = "%s ret = %s;" % (self.rv_type().cpptype, call_name)
        else:
            code = self.rv_type().cpp_method_call_invoke(call_name, ", ".join(call_cpp_args), self.is_constructor())
        code += "\n"

        # return value
        code += self.rv_type().cpp_method_return(self.is_constructor())

        template_vars = combine_dicts(self.__dict__, {
            "repr": repr(self),
            "rv_type": self.rv_type(),
            "args": args,
            "return_wrapper_type": self.rv_type().rust_cpp_return_wrapper_type(),
            "c_name": self.c_name(),
            "decl_cpp_args": ", ".join(decl_cpp_args),
            "code": indent(code, 2),
        })

        manual = self._get_manual_implementation("cpp", template_vars)
        if manual is None:
            self.rv_type().gen_return_wrappers(self.gen.cpp_dir, self.gen.rust_dir)
            return FuncInfo.TEMPLATES["cpp"].substitute(template_vars)
        else:
            return manual

    def gen_rust_extern(self):
        args = []
        if self.is_instance_method():
            args.append(self.ci.type_info().rust_extern_self_func_decl(not self.is_const))
        for a in self.args:
            args.append(a.type.rust_extern_arg_func_decl(a.rsname))
        template_vars = {
            "c_name": self.c_name(),
            "args": ", ".join(args),
            "return_wrapper_type": self.rv_type().rust_cpp_return_wrapper_type(),
        }
        manual = self._get_manual_implementation("rust_externs", template_vars)
        return FuncInfo.TEMPLATES["rust_externs"].substitute(template_vars) if manual is None else manual

    def gen_safe_rust(self):
        args = []
        call_args = []
        forward_args = []
        pre_call_args = []

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
            gdecl = arg.type.rust_generic_decl()
            if gdecl:
                generic_decls.append(gdecl)
            if self.has_callback_arg and arg.name == "userdata":
                continue
            args.append(arg.type.rust_arg_func_decl(arg.rsname, arg.is_output()))

        pub = "" if self.ci and self.ci.type_info().is_trait and not self.is_static else "pub "

        doc_comment = self.gen.reformat_doc(self.comment)

        first = True
        for arg in (x for x in self.args if x.defval != ""):
            if first:
                doc_comment += "///\n/// ## C++ default parameters:\n"
                first = False
            doc_comment += "/// * %s: %s\n" % (arg.rsname, arg.defval)
        template_vars = combine_dicts(self.__dict__, {
            "doc_comment": doc_comment,
            "rv_rust_full": self.rv_type().rust_full,
            "visibility": pub,
            "generic_decl": "<{}>".format(", ".join(generic_decls)) if len(generic_decls) >= 1 else "",
            "args": ", ".join(args),
            "pre_call_args": "".join("\n" + indent(x) + ";" for x in pre_call_args),
            "r_name": self.r_name(),
            "c_name": self.c_name(),
            "call_args": ", ".join(call_args),
            "forward_args": ", ".join(forward_args),
        })
        if isinstance(self.rv_type(), StringTypeInfo) or isinstance(self.rv_type(), RawPtrTypeInfo) and self.rv_type().is_string():
            if self.rv_type().is_const:
                rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_string"].substitute(template_vars)
            else:
                rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_string_mut"].substitute(template_vars)
        elif isinstance(self.rv_type(), RawPtrTypeInfo):
                rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_const_raw_ptr" if self.rv_type().is_const else "rust_safe_rv_mut_raw_ptr"].substitute(template_vars)
        elif self.rv_type().is_by_ptr:
            rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_vector_box_ptr"].substitute(template_vars)
        else:
            rv_rust = FuncInfo.TEMPLATES["rust_safe_rv_other"].substitute(template_vars)

        template_vars["rv"] = rv_rust

        block = self._get_manual_implementation("rust_safe", template_vars)
        if block is None:
            block = FuncInfo.TEMPLATES["rust_safe"].substitute(template_vars)

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
        self.ctype = decl[0]
        self.name = decl[1]
        self.rw = "/RW" in decl[3]

    def __repr__(self):
        return template("PROP $ctype $name").substitute(ctype=self.ctype, name=self.name)


class ClassInfo(GeneralInfo):
    def __init__(self, gen, module, decl, namespaces):  # [ 'class/struct cname', ': base', [modlist] ]
        """
        :type gen: RustWrapperGenerator
        :type module: str
        :type decl: list
        :type namespaces: frozenset
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.methods = []  # type: list[FuncInfo]
        self.namespaces = namespaces
        self.module = module
        self.is_simple = self.is_ignored = self.is_ghost = self.is_callback = False
        self.is_trait = self.fullname in forced_trait_classes
        self.classname = self.name
        self.comment = ""
        if len(decl) > 5:
            self.comment = decl[5].encode("ascii", "ignore")
        for m in decl[2]:
            if m == "/Simple" or m == "/Map":
                self.is_simple = True
            if m == "/Hidden":
                self.is_ignored = True
            if m == "/Ghost":
                self.is_ghost = True
            if m == "/Callback":
                self.is_callback = True
        if self.classpath and self.gen.get_class(self.classpath).is_ignored:
            self.is_ignored = True

        self.nested_cname = self.fullname.replace("::", "_")

        bases = decl[1][1:].strip()
        if len(bases):
            self.bases = [x for x in set(x.strip() for x in bases.split(",")) if x != self.fullname]
        else:
            self.bases = []

        for base in self.bases:
            typ = self.gen.get_class(base)
            if typ:
                typ.is_trait = True

        # class props
        self.props = []
        for p in decl[3]:
            self.props.append(ClassPropInfo(p))

        self.is_ignored = self.is_ignored or self.gen.class_is_ignored(self.fullname)

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

    def get_manual_declaration_template(self, section):
        if self.module in type_manual_declaration:
            module_types = type_manual_declaration[self.module]
            if self.name in module_types:
                templ = module_types[self.name]
                if section in templ:
                    if templ[section] == "~":
                        return None
                    return templ[section]
                else:
                    return template("")
        return None


class ConstInfo(GeneralInfo):
    TEMPLATES = {
        "rust_string": template("${doccomment}pub const ${name}: &'static str = ${value};\n"),
        "rust_int": template("${doccomment}pub const ${name}: i32 = ${value};\n"),
        "cpp_string": template("""    printf("pub static ${name}: &'static str = \\"%s\\";\\n", ${full_name});\n"""),
        "cpp_double": template("""    printf("pub const ${name}: f64 = %f;\\n", ${full_name});\n"""),
        "cpp_int": template("""    printf("pub const ${name}: i32 = 0x%x; // %i\\n", ${full_name}, ${full_name});\n"""),
    }

    def __init__(self, gen, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type decl: list
        :type namespaces: frozenset
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        _, self.rustname = split_known_namespace(self.fullname, namespaces)
        self.rustname = self.rustname.replace("::", "_")
        self.cname = self.name.replace(".", "::")
        self.value = decl[1]

    def __repr__(self):
        return template("CONST $name=$value").substitute(name=self.name, value=self.value)

    def is_ignored(self):
        for c in const_ignore_list:
            if re.match(c, self.name):
                return True
        return False

    def gen_rust(self):
        name = self.rustname
        value = self.value
        while True:
            doccomment = ""
            m = re.match(r"^(.+?)\s*(?://\s*(.+)|/\*+\s*(.+?)\s*\*+/)$", value)  # xxx // comment OR xxx /** comment **/
            if m:
                value = m.group(1)
                doccomment = "/// {}\n".format(m.group(3) if m.group(2) is None else m.group(2))
            if value.startswith('"'):
                return ConstInfo.TEMPLATES["rust_string"].substitute(doccomment=doccomment, name=name, value=value)
            elif re.match(r"^(-?[0-9]+|0x[0-9A-Fa-f]+)$", value):  # decimal or hexadecimal
                return ConstInfo.TEMPLATES["rust_int"].substitute(doccomment=doccomment, name=name, value=value)
            elif re.match(r"^\(?\s*(\d+\s*<<\s*\d+)\s*\)?$", value):  # (1 << 24)
                return ConstInfo.TEMPLATES["rust_int"].substitute(doccomment=doccomment, name=name, value=value)
            elif re.match(r"^\s*(\d+\s*\+\s*\d+)\s*$", value):  # 0 + 3
                return ConstInfo.TEMPLATES["rust_int"].substitute(doccomment=doccomment, name=name, value=value)
            ref_const = self.gen.get_const(value)
            if ref_const is not None:
                value = ref_const.value
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
        :type namespaces: frozenset
        """
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.alias = decl[1]
        self.comment = ""
        if len(decl) > 5:
            self.comment = decl[5].encode("ascii", "ignore")

    def typ(self):
        return self.gen.get_type_info(self.name)

    def alias_typ(self):
        return self.gen.get_type_info(self.alias)


class CallbackInfo(GeneralInfo):
    TEMPLATES = {
        "rust": template("""
        ${doc_comment}pub type ${name}Extern = Option<extern "C" fn(${extern_args})>;
        ${doc_comment}pub type ${name} = dyn FnMut(${args}) + Send + Sync + 'static;
        
        """),
    }

    def __init__(self, gen, decl, namespaces):
        """
        :type gen: RustWrapperGenerator
        :type decl: list
        :type namespaces: frozenset
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
            self.comment = decl[5].encode("ascii", "ignore")
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


class TypeInfo(object):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        self.is_const = typeid.startswith("const ")  # type has C++ const modifier
        self.typeid = typeid.replace("const ", "")  # e.g. "vector<cv::Mat>", "std::vector<int>", "float"
        self.gen = gen
        self.is_ignored = False  # don't generate
        # False: types that contain ptr field to actual heap allocated data (e.g. BoxedClass, Vector, SmartPtr)
        # True: types that are getting passed by value (e.g. Primitive, SimpleClass)
        self.is_by_ptr = False
        self.is_trait = False  # don't generate struct, generate trait (abstract classes and classes inside forced_trait_classes)

        self.cpp_extern = ""  # cpp type used on the boundary between Rust and C (e.g. in return wrappers)
        self.cpptype = self.typeid
        self.c_safe_id = "XX"  # c safe type identifier used for file names and return wrappers

        _, self.rust_local = split_known_namespace(self.typeid, gen.namespaces)
        self.rust_local = self.rust_local.replace("::", "_")  # only the type name for Rust without module path
        self.rust_safe_id = self.rust_local  # rust safe type identifier used for file and function names
        self.rust_full = ""  # full module path (with modules/crate::) to Rust type
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
                // $typeid
                pub type ${return_wrapper_type} = cv_return_value<crate::types::Unit, ${rust_extern}>;
                
            """),

            "rust_non_void": template("""
                // $typeid
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
        with open("{}/{}.type.h".format(cpp_dir, template_vars["return_wrapper_type"]), "w") as f:
            f.write(self.base_templates["cpp_void" if self.cpp_extern == "void" else "cpp_non_void"].substitute(template_vars))
        with open("{}/{}.rv.rs".format(rust_dir, template_vars["return_wrapper_type"]), "w") as f:
            f.write(self.base_templates["rust_void" if self.cpp_extern == "void" else "rust_non_void"].substitute(template_vars))

    def rust_arg_forward(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return var_name

    def rust_arg_pre_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return ""

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

    def rust_generic_decl(self):
        return ""

    def rust_arg_func_decl(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        if self.is_by_ptr:
            if is_output:
                return "{}: &mut {}".format(var_name, self.rust_full)
            return "{}: &{}".format(var_name, self.rust_full)
        return "{}: {}".format(var_name, self.rust_full)

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

    def rust_self_func_call(self, is_output=False):
        """
        :type is_output: bool
        :rtype: str
        """
        return self.rust_arg_func_call("self", is_output)

    def rust_extern_arg_func_decl(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        return "{}: {}".format(var_name, self.rust_extern)

    def rust_extern_self_func_decl(self, is_output=False):
        """
        :type is_output: bool
        :rtype: str
        """
        if self.is_by_ptr:
            return "instance: *{} c_void".format("mut" if is_output else "const")
        return "instance: {}".format(self.rust_full)

    def cpp_arg_func_decl(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        if is_output:
            return "{}* {}".format(self.cpp_extern, var_name)
        return "{} {}".format(self.cpp_extern, var_name)

    def cpp_arg_func_call(self, var_name, is_output=False):
        """
        :type var_name: str
        :type is_output: bool
        :rtype: str
        """
        if self.is_by_ptr:
            return "*reinterpret_cast<{}*>({})".format(self.cpptype, var_name)
        return "*reinterpret_cast<{}*>(&{})".format(self.cpptype, var_name)

    def cpp_method_call_name(self, method_name):
        """
        :type method_name: str
        :rtype: str
        """
        return "reinterpret_cast<{}*>(&instance)->{}".format(self.cpptype, method_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor):
        """
        :type call_name: str
        :type call_args: str
        :type is_constructor: bool
        :rtype: str
        """
        if is_constructor:
            if call_args == "":
                return "{} ret;".format(self.cpptype)
            return "{} ret({});".format(self.cpptype, call_args)
        return "{} ret = {}({});".format(self.cpptype, call_name, call_args)

    def cpp_method_return(self, is_constructor):
        if self.is_by_ptr:
            return "return { Error::Code::StsOk, NULL, ret };"
        return "return {{ Error::Code::StsOk, NULL, *reinterpret_cast<{}*>(&ret) }};".format(self.cpp_extern)

    def rust_cpp_return_wrapper_type(self):
        """
        :rtype: str
        """
        return "cv_return_value_{}".format(self.c_safe_id)


class StringTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(StringTypeInfo, self).__init__(gen, typeid)
        self.cpp_extern = "const char*"
        self.cpptype = "String"
        self.rust_full = "String"
        if self.is_const:
            self.c_safe_id = "const_char_X"
            self.rust_local = "*const c_char"  # fixme, not used?
            self.rust_extern = "*const c_char"
        else:
            self.c_safe_id = "char_X"
            self.rust_local = "*mut c_char"  # fixme, not used?
            self.rust_extern = "*mut c_char"
        self.rust_safe_id = "String"

    def cpp_arg_func_call(self, var_name, is_output=False):
        return "{}({})".format(self.cpptype, var_name)

    def rust_arg_pre_call(self, var_name, is_output=False):
        return "string_arg!({}{})".format("" if self.is_const else "mut ", var_name)

    def rust_arg_func_call(self, var_name, is_output=False):
        if self.is_const:
            return "{}.as_ptr()".format(var_name)
        return "{}.as_ptr() as _".format(var_name)  # fixme: use as_mut_ptr() when it's stabilized

    def rust_arg_func_decl(self, var_name, is_output=False):
        return "{}: &str".format(var_name)

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
        super(IgnoredTypeInfo, self).__init__(gen, typeid)
        self.is_ignored = True

    def __str__(self):
        return "Ignored(%s)"%(self.typeid)


class PrimitiveTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(PrimitiveTypeInfo, self).__init__(gen, typeid)
        primitive = primitives[self.typeid]
        self.cpp_extern = primitive["cpp_extern"]
        self.rust_extern = self.rust_full = self.rust_local = primitive["rust_local"]
        self.rust_safe_id = self.typeid.replace(" ", "_")
        self.c_safe_id = self.cpp_extern.replace(" ", "_").replace("*", "X").replace("::", "_")

    def cpp_arg_func_call(self, var_name, is_output=False):
        return var_name

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor):
        if self.cpptype == "void":
            return "{}({});".format(call_name, call_args)
        return super(PrimitiveTypeInfo, self).cpp_method_call_invoke(call_name, call_args, is_constructor)

    def cpp_method_return(self, is_constructor):
        if self.cpptype == "void":
            return "return { Error::Code::StsOk, NULL };"
        return super(PrimitiveTypeInfo, self).cpp_method_return(is_constructor)

    def __str__(self):
        return "Primitive(%s)" % (self.cpptype)


class SimpleClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(SimpleClassTypeInfo, self).__init__(gen, typeid)
        self.ci = gen.get_class(self.typeid)
        if self.ci and self.ci.is_ignored:
            self.is_ignored = True
        if self.ci:
            self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
            if self.ci.get_manual_declaration_template("rust") is None:
                self.cpp_extern = self.ci.fullname
                self.c_safe_id = self.rust_local
            else:
                self.cpp_extern = "{}Wrapper".format(self.ci.name)
                self.c_safe_id = self.cpp_extern
            self.rust_extern = self.rust_full
            self.is_trait = False

    def __str__(self):
        return "%s (simple)"%(self.cpptype)


class CallbackTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(CallbackTypeInfo, self).__init__(gen, typeid)
        self.ci = gen.get_class(self.typeid)
        if self.ci and self.ci.is_ignored:
            self.is_ignored = True
        if self.ci:
            self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
            self.cpp_extern = self.ci.fullname
            self.c_safe_id = self.rust_local
            self.rust_extern = "{}Extern".format(self.rust_full)
            self.is_trait = False

    def rust_arg_pre_call(self, var_name, is_output=False):
        callback_info = self.gen.get_callback(self.typeid)
        if callback_info is None or callback_info.is_ignored:
            return super(CallbackTypeInfo, self).rust_generic_decl()
        extern_args = []
        rust_args = []
        for arg in callback_info.args:
            extern_args.append(arg.type.rust_extern_arg_func_decl(arg.rsname, arg.is_output()))
            if arg.name != "userdata":
                rust_args.append(arg.type.rust_arg_func_decl(arg.rsname, arg.is_output()))
        return "callback_arg!({}({}) via userdata => ({}))".format(var_name, ", ".join(extern_args), ", ".join(rust_args))

    def rust_arg_func_decl(self, var_name, is_output=False):
        callback_info = self.gen.get_callback(self.typeid)
        if callback_info is None or callback_info.is_ignored:
            return super(CallbackTypeInfo, self).rust_arg_func_decl(var_name, is_output)
        return "{}: Option<Box<{}>>".format(var_name, self.rust_full)

    def __str__(self):
        return "{} (callback)".format(self.cpptype)


class BoxedClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(BoxedClassTypeInfo, self).__init__(gen, typeid)
        self.ci = gen.get_class(self.typeid)
        self.cpptype = self.ci.fullname
        self.rust_extern = "*mut c_void"
        self.rust_full = ("crate::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
        self.is_by_ptr = True
        self.is_trait = self.typeid in forced_trait_classes or self.ci.is_trait
        self.cpp_extern = "void*"
        self.c_safe_id = "void_X"
        self.is_ignored = self.ci.is_ignored
        self.rust_safe_id = self.ci.name

    def cpp_arg_func_decl(self, var_name, is_output=False):
        return super(BoxedClassTypeInfo, self).cpp_arg_func_decl(var_name, False)

    def cpp_method_call_name(self, method_name):
        return "reinterpret_cast<{}*>(instance)->{}".format(self.cpptype, method_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor):
        if is_constructor:
            return "{}* ret = new {}({});".format(self.cpptype, call_name, call_args)
        return super(BoxedClassTypeInfo, self).cpp_method_call_invoke(call_name, call_args, is_constructor)

    def cpp_method_return(self, is_constructor):
        if not is_constructor:
            return "return {{ Error::Code::StsOk, NULL, new {}(ret) }};".format(self.cpptype)
        return super(BoxedClassTypeInfo, self).cpp_method_return(is_constructor)

    def __str__(self):
        return "%s (boxed)"%(self.typeid)


class VectorTypeInfo(TypeInfo):
    TEMPLATES = {  # fixme _delete to suffix
        "rust_common": template("""
                extern "C" {
                   #[doc(hidden)] fn cv_new_${rust_safe_id}() -> ${rust_extern};
                   #[doc(hidden)] fn cv_delete_${rust_safe_id}(ptr: ${rust_extern});
                   #[doc(hidden)] fn cv_push_${rust_safe_id}(ptr: ${rust_extern}, ptr2: *const c_void);
                   #[doc(hidden)] fn cv_${rust_safe_id}_len(ptr: ${rust_extern}) -> i32;
                   #[doc(hidden)] fn cv_${rust_safe_id}_get(ptr: ${rust_extern}, index: i32) -> ${rust_extern};
                }
                
                #[allow(dead_code)]
                pub struct ${rust_local} {
                    pub(crate) ptr: ${rust_extern}
                }
                
                impl ${rust_local} {
                    pub fn new() -> Self {
                        unsafe { Self { ptr: cv_new_${rust_safe_id}() } }
                    }
                    
                    pub fn len(&self) -> i32 {
                        unsafe { cv_${rust_safe_id}_len(self.ptr) }
                    }
                    
                    #[doc(hidden)]
                    pub fn as_raw_$rust_local(&self) -> ${rust_extern} {
                        self.ptr
                    }
                }

                impl Drop for $rust_local {
                    fn drop(&mut self) {
                        unsafe { cv_delete_${rust_safe_id}(self.ptr) };
                    }
                }
            """),

        "rust_boxed": template("""
                // BoxedClassTypeInfo
                impl ${rust_local} {
                    pub fn push(&mut self, val: ${inner_rust_full}) {
                        unsafe { cv_push_${rust_safe_id}(self.ptr, val.ptr) }
                    }
                    
                    pub fn get(&self, index: i32) -> ${inner_rust_full} {
                        ${inner_rust_full} { ptr: unsafe { cv_${rust_safe_id}_get(self.ptr, index) } }
                    }
                    
                    pub fn to_vec(&self) -> Vec<$inner_rust_full> {
                        (0..self.len()).map(|x| self.get(x)).collect()
                    }
                }
            """),

        "rust_non_boxed": template("""
                impl ${rust_local} {
                    pub fn push(&mut self, val: ${inner_rust_full}) {
                        unsafe { cv_push_${rust_safe_id}(self.ptr, &val as *const _ as _) }
                    }
                    
                    pub fn get(&self, index: i32) -> &mut $inner_rust_full {
                        unsafe { (cv_${rust_safe_id}_get(self.ptr, index) as *mut ${inner_rust_full}).as_mut().unwrap() }
                    }
                }
            """),

        "rust_non_bool": template("""
                extern "C" { #[doc(hidden)] fn cv_${rust_safe_id}_data(ptr: ${rust_extern}) -> ${rust_extern}; }
                
                impl ::std::ops::Deref for ${rust_local} {
                    type Target = [${inner_rust_full}];
                    
                    fn deref(&self) -> &Self::Target {
                        unsafe {
                            let length = cv_${rust_safe_id}_len(self.ptr) as usize;
                            let data = cv_${rust_safe_id}_data(self.ptr);
                            ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
                        }
                    }
                }
            """),

        "cpp_externs": template("""
                    void* cv_new_${rust_safe_id}() {
                        return new ${cpptype}();
                    }
                    
                    void cv_delete_${rust_safe_id}(void* ptr) {
                        delete reinterpret_cast<${cpptype}*>(ptr);
                    }
                    
                    void cv_push_${rust_safe_id}(void* ptr, void* ptr2) {
                        ${inner_cpptype}* val = reinterpret_cast<${inner_cpptype}*>(ptr2);
                        reinterpret_cast<${cpptype}*>(ptr)->push_back(*val);
                    }
                    
                    ${inner_cpptype}* cv_${rust_safe_id}_front_item(void* ptr) {
                        ${inner_cpptype} val = reinterpret_cast<${cpptype}*>(ptr)->front();
                        return new ${inner_cpptype}(val);
                    }
                    
                    ${inner_cpptype}* cv_${rust_safe_id}_back_item(void* ptr) {
                        ${inner_cpptype} val = reinterpret_cast<${cpptype}*>(ptr)->back();
                        return new ${inner_cpptype}(val);
                    }
                    
                    int cv_${rust_safe_id}_len(void* ptr) {
                        return reinterpret_cast<${cpptype}*>(ptr)->size();
                    }
            """),

        "cpp_externs_bool": template("""
                ${inner_cpptype}* cv_${rust_safe_id}_get(void* ptr, int index) {
                    ${inner_cpptype} val = (*reinterpret_cast<${cpptype}*>(ptr))[index];
                    return new ${inner_cpptype}(val);
                }
            """),

        "cpp_externs_non_bool": template("""
                ${inner_cpptype}* cv_${rust_safe_id}_get(void* ptr, int index) {
                    ${inner_cpptype} val = reinterpret_cast<${cpptype}*>(ptr)->data()[index];
                    return new ${inner_cpptype}(val);
                }
                
                ${cpp_extern}* cv_${rust_safe_id}_data(void* ptr) {
                    return reinterpret_cast<${cpp_extern}*>(reinterpret_cast<${cpptype}*>(ptr)->data());
                }
            """),

    }

    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(VectorTypeInfo, self).__init__(gen, typeid)
        self.is_by_ptr = True
        self.inner = inner
        if isinstance(self.inner, RawPtrTypeInfo):  # fixme, lifetimes required
            self.is_ignored = True
        else:
            self.is_ignored = inner.is_ignored
        if not self.is_ignored:
            self.cpp_extern = "void*"
            self.c_safe_id = "void_X"
            self.inner_cpptype = inner.cpptype
            if isinstance(self.inner, SmartPtrTypeInfo):
                self.outer_cpptype = "std::vector<%s>" % (self.inner.outer_cpptype)
                self.inner_outer_cpptype = self.inner.outer_cpptype
            self.cpptype = "std::vector<%s>" % (inner.cpptype)
            self.rust_safe_id = self.rust_local = "VectorOf"+inner.rust_safe_id
            self.rust_full = "types::" + self.rust_local
            self.rust_extern = "*mut c_void"
            self.inner_rust_full = inner.rust_full

    def gen_wrappers(self):
        with open("{}/{}.type.rs".format(self.gen.rust_dir, self.rust_safe_id), "w") as f:
            f.write(VectorTypeInfo.TEMPLATES["rust_common"].substitute(self.__dict__))
            if isinstance(self.inner, BoxedClassTypeInfo) or self.inner.is_by_ptr:
                f.write(VectorTypeInfo.TEMPLATES["rust_boxed"].substitute(self.__dict__))
            else:
                f.write(VectorTypeInfo.TEMPLATES["rust_non_boxed"].substitute(self.__dict__))
                if self.inner.typeid != "bool":
                    f.write(VectorTypeInfo.TEMPLATES["rust_non_bool"].substitute(self.__dict__))

        if isinstance(self.inner, SmartPtrTypeInfo):
            template_vars = self.__dict__.copy()
            template_vars["cpptype"] = self.outer_cpptype
            template_vars["inner_cpptype"] = self.inner_outer_cpptype
        else:
            template_vars = self.__dict__
        externs = VectorTypeInfo.TEMPLATES["cpp_externs"].substitute(template_vars)
        if self.inner.typeid == "bool":
            externs += VectorTypeInfo.TEMPLATES["cpp_externs_bool"].substitute(template_vars)
        else:
            externs += VectorTypeInfo.TEMPLATES["cpp_externs_non_bool"].substitute(template_vars)
        with open("{}/{}.type.cpp".format(self.gen.cpp_dir, self.rust_safe_id), "w") as f:
            f.write(T_CPP_MODULE.substitute(code=externs, includes=""))

    def cpp_arg_func_call(self, var_name, is_output=False):
        if isinstance(self.inner, SmartPtrTypeInfo):
            return "*reinterpret_cast<{}*>({})".format(self.outer_cpptype.replace("&", ""), var_name)
        return super(VectorTypeInfo, self).cpp_arg_func_call(var_name, is_output)

    def cpp_method_return(self, is_constructor):
        return "return {{ Error::Code::StsOk, NULL, (void *)new {}(ret) }};".format(self.cpptype)

    def __str__(self):
        return "Vector[%s]" % (self.inner)


class SmartPtrTypeInfo(TypeInfo):
    TEMPLATES = {
        "rust": template("""
                extern "C" {
                    #[doc(hidden)] fn cv_${rust_safe_id}_get(ptr: ${rust_extern}) -> ${rust_extern};
                    #[doc(hidden)] fn cv_delete_${rust_safe_id}(ptr: ${rust_extern});
                }

                #[allow(dead_code)]
                pub struct ${rust_local} {
                    pub(crate) ptr: ${rust_extern}
                }

                impl ${rust_local} {
                    #[doc(hidden)] pub fn as_raw_${rust_safe_id}(&self) -> ${rust_extern} {
                        self.ptr
                    }
                }

                impl Drop for ${rust_local} {
                    fn drop(&mut self) {
                        unsafe { cv_delete_${rust_safe_id}(self.ptr) };
                    }
                }
            """),

        "rust_trait_cast": template("""
                impl ${base_full} for ${rust_local} {
                    #[doc(hidden)]
                    fn as_raw_${base_local}(&self) -> ${rust_extern} {
                        unsafe { cv_${rust_safe_id}_get(self.ptr) }
                    }
                }
                
            """),

        "cpp_externs": template("""
                void* cv_${rust_safe_id}_get(${cpp_extern} ptr) {
                    return reinterpret_cast<${outer_cpptype}*>(ptr)->get();
                }
                void  cv_delete_${rust_safe_id}(${cpp_extern} ptr) {
                    delete reinterpret_cast<${outer_cpptype}*>(ptr);
                }
            """),
    }

    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        super(SmartPtrTypeInfo, self).__init__(gen, typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = self.inner.is_ignored
        if not self.is_ignored:
            self.cpp_extern = "void*"
            self.c_safe_id = "void_X"
            self.rust_extern = "*mut c_void"
            self.cpptype = self.inner.cpptype
            self.outer_cpptype = "Ptr<{}>".format(self.inner.cpptype)
            self.rust_local = self.rust_safe_id = "PtrOf{}".format(inner.rust_safe_id)
            self.rust_full = "types::{}".format(self.rust_local)
            self.inner_rust_full = inner.rust_full
            self.inner_local = inner.rust_local

    def gen_wrappers(self):
        with open("{}/{}.type.rs".format(self.gen.rust_dir, self.rust_safe_id), "w") as f:
            f.write(SmartPtrTypeInfo.TEMPLATES["rust"].substitute(self.__dict__))
            if not isinstance(self.inner, PrimitiveTypeInfo) and self.inner.ci.is_trait:
                bases = self.gen.all_bases(self.inner.ci.name).union({self.inner.typeid})
                for base in bases:
                    cibase = self.gen.get_type_info(base)
                    if not isinstance(cibase, UnknownTypeInfo):
                        f.write(SmartPtrTypeInfo.TEMPLATES["rust_trait_cast"].substitute(
                            rust_local=self.rust_local,
                            rust_safe_id=self.rust_safe_id,
                            rust_extern=self.rust_extern,
                            base_local=cibase.rust_local,
                            base_full=cibase.rust_full
                        ))
        with open("{}/{}.type.cpp".format(self.gen.cpp_dir, self.rust_safe_id), "w") as f:
            code = SmartPtrTypeInfo.TEMPLATES["cpp_externs"].substitute(self.__dict__)
            f.write(T_CPP_MODULE.substitute(code=code, includes=""))

    def cpp_arg_func_call(self, var_name, is_output=False):
        return "reinterpret_cast<{}*>({})".format(self.cpptype, var_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor):
        return "Ptr<{}> *ret = new Ptr<{}>({}({}));".format(self.cpptype, self.cpptype, call_name, call_args)

    def __str__(self):
        return "SmartPtr[%s]" % (self.inner)


class RawPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        super(RawPtrTypeInfo, self).__init__(gen, typeid)
        self.inner = inner
        if self.inner.is_ignored or isinstance(self.inner, RawPtrTypeInfo):
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
                if self.is_const:
                    pass
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
                elif self.is_string():
                    self.rust_full = "String"
                    self.rust_extern += "c_char"
                else:
                    self.rust_full += self.inner.rust_full
                    self.rust_extern += self.inner.rust_extern
            if self.is_const:
                self.c_safe_id = "const_" + self.c_safe_id
                self.cpptype = "const " + self.cpptype
                self.cpp_extern = "const " + self.cpp_extern
                self.rust_safe_id = "const_" + self.rust_safe_id

    def is_string(self):
        return isinstance(self.inner, PrimitiveTypeInfo) and self.inner.cpptype == "char"

    def rust_arg_func_decl(self, var_name, is_output=False):
        if self.is_string():
            return "{}: &{}str".format(var_name, "mut " if is_output else "")
        return super(RawPtrTypeInfo, self).rust_arg_func_decl(var_name, is_output)

    def rust_arg_pre_call(self, var_name, is_output=False):
        if self.is_string():
            return "string_arg!({})".format(var_name)
        return super(RawPtrTypeInfo, self).rust_arg_pre_call(var_name, is_output)

    def rust_arg_func_call(self, var_name, is_output=False):
        if self.is_string():
            if self.is_const:
                return "{}.as_ptr()".format(var_name)
            return "{}.as_ptr() as _".format(var_name)  # fixme: use as_mut_ptr() when it's stabilized
        return super(RawPtrTypeInfo, self).rust_arg_func_call(var_name, is_output)

    def cpp_arg_func_call(self, var_name, is_output=False):
        if isinstance(self.inner, PrimitiveTypeInfo):
            return var_name
        if self.is_by_ptr:
            return "reinterpret_cast<{}*>({})".format(self.cpptype, var_name)
        return "reinterpret_cast<{}*>(&{})".format(self.inner.cpptype, var_name)

    def cpp_method_call_invoke(self, call_name, call_args, is_constructor):
        if self.is_by_ptr:
            return "{}* ret = {}({});".format(self.cpptype, call_name, call_args)
        return super(RawPtrTypeInfo, self).cpp_method_call_invoke(call_name, call_args, is_constructor)

    def cpp_method_return(self, is_constructor):
        if self.is_string():
            return "return { Error::Code::StsOk, NULL, strdup(ret) };"
        return super(RawPtrTypeInfo, self).cpp_method_return(is_constructor)

    def __str__(self):
        return "RawPtr[%s]" % (self.inner)


class UnknownTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        super(UnknownTypeInfo, self).__init__(gen, typeid)
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
    if full_typeid.startswith("const "):
        typeid = full_typeid[6:]
    if typeid == "":
        typeid = "void"
        full_typeid = "void"
    # if typeid.endswith("&"):
    #     return ReferenceTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    if typeid.endswith("&"):
        typeid = typeid[:-1].strip()
        full_typeid = full_typeid[:-1].strip()
    if typeid in primitives:
        return PrimitiveTypeInfo(gen, full_typeid)
    elif typeid.endswith("*"):
        return RawPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[:-1].strip()))
    elif typeid.endswith("[]"):
        return RawPtrTypeInfo(gen, full_typeid, gen.get_type_info(typeid[:-2].strip()))
    elif typeid == "string" or typeid == "String":
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
        ci = gen.get_class(typeid)
        if ci and not ci.is_ignored:
            if ci.is_simple:
                return SimpleClassTypeInfo(gen, ci.fullname)
            elif ci.is_callback:
                return CallbackTypeInfo(gen, ci.fullname)
            else:
                return BoxedClassTypeInfo(gen, ci.fullname)
        actual = type_replace.get(typeid)
        if actual:
            ci = gen.get_class(actual)
            if ci:
                if ci.is_simple:
                    return SimpleClassTypeInfo(gen, ci.fullname)
                elif ci.is_callback:
                    return CallbackTypeInfo(gen, ci.fullname)
                else:
                    return BoxedClassTypeInfo(gen, ci.fullname)
            return parse_type(gen, actual)
    return UnknownTypeInfo(gen, full_typeid)

#
#       GENERATOR
#


class RustWrapperGenerator(object):
    TEMPLATES = {
        "simple_class": {
            "rust_struct": template("""
                    ${doc_comment}
                    #[repr(C)]
                    #[derive(Copy,Clone,Debug,PartialEq)]
                    pub struct ${rust_local} {
                    ${fields}
                    }
                    
                """),

            "rust_struct_simple": template("""
                    ${doc_comment}
                    #[repr(C)]
                    #[derive(Copy,Clone,Debug,PartialEq)]
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
    }

    def __init__(self):
        self.cpp_dir = ""
        self.rust_dir = ""
        self.classes = OrderedDict()  # type: dict[str, ClassInfo]
        self.functions = []
        self.ported_func_list = []
        self.skipped_func_list = []
        self.consts = []
        self.type_infos = {}
        self.callbacks = []  # type: list[CallbackInfo]
        self.namespaces = set()
        self.generated = set()
        self.generated_functions = []
        self.func_names = set()

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

    def add_decl(self, module, decl):
        if decl[0] == "cv.String.String" or decl[0] == 'cv.Exception.~Exception':
            return
        if decl[0] == "cv.Algorithm":
            decl[0] = "cv.Algorithm.Algorithm"
        name = decl[0]  # type: str
        if name.startswith("struct") or name.startswith("class"):
            self.add_class_decl(module, decl)
        elif name.startswith("const"):
            self.add_const_decl(module, decl)
        elif name.startswith("typedef"):
            self.add_typedef_decl(module, decl)
        elif name.startswith("callback"):
            self.add_callback_decl(module, decl)
        else:
            self.add_func_decl(module, decl)

    def add_class_decl(self, module, decl):
        item = ClassInfo(self, module, decl, frozenset(self.namespaces))
        # register
        logging.info("register class %s (%s)%s%s", item.fullname, decl,
                     " [ignored]" if item.is_ignored else "",
                     " impl:"+",".join(item.bases) if len(item.bases) else "")
        self.classes[item.fullname] = item
        """
        This is used to create methods, what allows you to access to simple types inside
        complex structures. For instance, RotatedRect items contains Point2f, Size and angle instances,
        this method creates attribute getters get_point, get_size, get_angle.
        """
        for struct_field, field_type in boxed_type_fields.get(item.classname, {}).items():
            field_decl = [
                u'{classname}.get_{struct_field}'.format(
                    classname=item.classname,
                    struct_field=struct_field,
                ),
                field_type,
                ['/ATTRGETTER'],
                [],
                None,
                u'returns the {struct_field} attr of {field_type} type\n'.format(
                    struct_field=struct_field,
                    field_type=field_type,
                ),
                struct_field,
            ]
            item.add_method(FuncInfo(self, module, field_decl, frozenset(self.namespaces)))

    def add_const_decl(self, _module, decl):
        item = ConstInfo(self, decl, frozenset(self.namespaces))
        # register
        if item.is_ignored():
            logging.info('ignored: %s', item)
        elif not self.get_const(item.name):
            self.consts.append(item)

    def add_typedef_decl(self, _module, decl):
        item = TypedefInfo(self, decl, frozenset(self.namespaces))
        if not isinstance(item.alias_typ(), UnknownTypeInfo) and isinstance(item.typ(), UnknownTypeInfo):
            self.set_type_info(item.name, item.alias_typ())

    def add_callback_decl(self, _module, decl):
        item = CallbackInfo(self, decl, frozenset(self.namespaces))
        if not item.is_ignored:
            self.add_decl(_module, ("class {}".format(item.fullname.replace("::", ".")), "", ["/Ghost", "/Callback"], []))
            self.callbacks.append(item)

    def add_func_decl(self, module, decl):
        item = FuncInfo(self, module, decl, frozenset(self.namespaces))
        if not item.is_ignored:
            # register self to class or generator
            if item.kind == item.KIND_FUNCTION:
                self.register_function(item)
            else:
                item.ci.add_method(item)

    def register_function(self, f):
        logging.info("register %s %s (%s)"%(f.kind, f.name, f.identifier))
        self.functions.append(f)

    def gen(self, srcfiles, module, cpp_dir, rust_dir):
        """
        :param srcfiles:
        :type module: str
        :type cpp_dir: str
        :type rust_dir: str
        :return:
        """
        self.cpp_dir = cpp_dir
        self.rust_dir = rust_dir
        includes = []

        parser = hdr_parser.CppHeaderParser()
        self.namespaces = set(x for x in parser.namespaces)
        self.namespaces.add("cv")

        for m, decls in decls_manual_pre.iteritems():
            for decl in decls:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(m, decl)

        for hdr in srcfiles:
            decls = parser.parse(hdr, False)
            self.namespaces = set(str(x.replace(".", "::")) for x in parser.namespaces)
            logging.info("\n\n=============== Header: %s ================\n\n", hdr)
            logging.info("Namespaces: %s", parser.namespaces)
            logging.info("Comment: %s", parser.module_comment)
            includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                self.add_decl(module, decl)

        for m, decls in decls_manual_post.iteritems():
            for decl in decls:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(m, decl)

        logging.info("\n\n===== Generating... =====")
        self.moduleCppTypes = StringIO()
        self.moduleCppCode = StringIO()
        self.moduleCppConsts = StringIO()
        self.moduleSafeRust = StringIO()
        self.moduleRustExterns = StringIO()

        self.moduleSafeRust.write('//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>\n'.encode("utf-8"))
        self.moduleSafeRust.write(self.reformat_doc(parser.module_comment.get(module, ""), "//!"))

        self.moduleSafeRust.write(template("""
            use std::os::raw::{c_char, c_void};
            use libc::size_t;
            use crate::{Error, Result, """ + ", ".join(static_modules) + """};
            
        """).substitute())
        for co in sorted(self.consts, key=lambda c: c.rustname):
            rust = co.gen_rust()
            if rust:
                self.moduleSafeRust.write(rust)
            else:
                self.moduleCppConsts.write(co.gen_cpp_for_complex())

        self.moduleSafeRust.write("\n")

        for cb in self.callbacks:
            self.gen_callback(cb)

        for t in self.type_infos.values():
            if not t.is_ignored:
                t.gen_wrappers()

        for c in self.classes.values():
            if c.is_simple and not c.is_ignored and not c.is_ghost and c.module == module:
                self.gen_simple_class(c)

        for fi in sorted(self.functions, key=lambda fi: fi.identifier):
            if not fi.is_ignored:
                self.gen_func(fi)

#        if module in forced_boxed_classes:
#            for cb in sorted(forced_boxed_classes[module]):
#                self.gen_boxed_class(cb)

        for ci in sorted(self.classes.values(), key=lambda ci:ci.fullname):
            self.gen_class(ci)

        with open(cpp_dir + "/types.h", "a") as f:
            f.write(self.moduleCppTypes.getvalue())

        with open(cpp_dir + "/" + module + ".consts.cpp", "w") as f:
            f.write("""#include <cstdio>\n""")
            f.write("""#include "opencv2/opencv_modules.hpp"\n""")
            f.write("""#include "opencv2/%s.hpp"\n"""%(module))
            for include in includes:
                f.write(include+"\n")
            f.write("""using namespace cv;\n""")
            f.write("int main(int, char**) {\n")
            f.write(self.moduleCppConsts.getvalue())
            f.write("}\n")

        namespaces = ""
        for namespace in self.namespaces:
            if namespace != "":
                namespaces += "using namespace %s;\n"%(namespace.replace(".", "::"))
        with open(cpp_dir + "/" + module + ".cpp", "w") as f:
            f.write(T_CPP_MODULE.substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes), namespaces=namespaces))

        with open(rust_dir + "/%s.externs.rs" % (module), "w") as f:
            f.write("extern \"C\" {\n")
            f.write(self.moduleRustExterns.getvalue())
            f.write("}\n")

        with open(rust_dir + "/" + module + ".rs", "w") as f:
            f.write(self.moduleSafeRust.getvalue())

        with open(cpp_dir + "/" + module + ".txt", "w") as f:
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

    def gen_vector_struct_for(self, name):
        struct_name = "cv_vector_of_"+name
        self.defined_in_types_h.appand(struct_name)
        self.moduleCppTypes.write

    def gen_func(self, fi):
        """
        :type fi: FuncInfo
        :return:
        """
        if fi.kind == fi.KIND_FUNCTION or fi.fake_attrgetter:
            for item in self.generated_functions:
                if item.name == fi.name and str(item.args) == str(fi.args):
                    return
            else:
                self.generated_functions.append(fi)
        logging.info("Generating func %s"%(fi.identifier))
        reason = fi.reason_to_skip()
        if reason:
            logging.info("  ignored: " + reason)
            self.skipped_func_list.append("%s\n   %s\n"%(fi,reason))
            return
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
        classname = "" if fi.kind == fi.KIND_FUNCTION else fi.classname
        renamed = False
        while classname + '::' + rust_func_name in self.func_names:
            rust_func_name = bump_counter(rust_func_name)
            renamed = True
        if renamed:
            renamed_funcs[fi.identifier] = rust_func_name

        # rust safe wrapper
        self.moduleSafeRust.write(fi.gen_safe_rust())
        self.func_names.add(classname + '::' + fi.r_name())

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

        templ = ci.get_manual_declaration_template("rust")
        if templ is None:
            templ = RustWrapperGenerator.TEMPLATES["simple_class"]["rust_struct"]
        self.moduleSafeRust.write(templ.substitute(combine_dicts(ci.type_info().__dict__, {
            "doc_comment": self.reformat_doc(ci.comment).rstrip(),
            "fields": indent(rust_fields.rstrip()),
        })))
        templ = ci.get_manual_declaration_template("cpp")
        if templ is None:
            templ = RustWrapperGenerator.TEMPLATES["simple_class"]["cpp_struct"]
        self.moduleCppTypes.write(templ.substitute(combine_dicts(ci.type_info().__dict__, {
            "fields": indent(cpp_fields.rstrip()),
        })))

    def gen_boxed_class(self, name):
        ci = self.get_class(name)
        if not ci:
            logging.info("type %s not found", name)
            return
        typ = ci.type_info()
        logging.info("Generating box for %s", ci)

        self.moduleCppCode.write(template("""
            // boxed class: $typeid
            void cv_delete_$rust_local(void* instance) {
                delete ($cpptype*) instance;
            }
            """).substitute(typ.__dict__))

        self.moduleRustExterns.write("#[doc(hidden)] pub fn cv_delete_%s(ptr : *mut c_void);\n" % (typ.rust_local))

        self.moduleSafeRust.write("// boxed class %s\n"%(typ.typeid))
        self.moduleSafeRust.write(self.reformat_doc(ci.comment))
        self.moduleSafeRust.write(template("""
            #[allow(dead_code)]
            pub struct $rust_local {
                #[doc(hidden)] pub ptr: *mut c_void
            }
            impl Drop for $rust_full {
                fn drop(&mut self) {
                    unsafe { sys::cv_delete_${rust_local}(self.ptr) };
                }
            }
            impl $rust_full {
                #[doc(hidden)] pub fn as_raw_${rust_local}(&self) -> *mut c_void { self.ptr }
            }
            
            """).substitute(typ.__dict__))

        bases = self.all_bases(name)
        for base in bases:
            cibase = self.get_class(base)
            if cibase is not None:
                cibase = cibase.type_info()
                self.moduleSafeRust.write(template("""
                    impl $base_full for ${rust_local} {
                        #[doc(hidden)] fn as_raw_$base_local(&self) -> *mut c_void { self.ptr }
                    }
                    
                """).substitute(rust_local=typ.rust_local, base_local=cibase.rust_local, base_full=cibase.rust_full))

    # all your bases...
    def all_bases(self, name):
        bases = set()
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
        t = ci.type_info()
        if not t:
            logging.info("Ignore class %s (not found)", ci)
            return
        if ci.namespace == "":
            logging.info("Not namespaced. Skipped %s", ci)
            return
        if t.is_trait:
            if len(ci.bases):
                bases = (x.rust_full for x in (self.get_type_info(x) for x in ci.bases) if not isinstance(x, UnknownTypeInfo))
                bases = " : " + " + ".join(bases)
            else:
                bases = ""
            logging.info("Generating impl for trait %s", ci)
            self.moduleSafeRust.write("// Generating impl for trait %s\n" % (ci))
            self.moduleSafeRust.write(self.reformat_doc(ci.comment.strip()))
            self.moduleSafeRust.write("pub trait %s%s {\n" % (t.rust_local, bases))
            self.moduleSafeRust.write("    #[doc(hidden)] fn as_raw_%s(&self) -> *mut c_void;\n" % (t.rust_local))
            for fi in ci.methods:
                if not fi.is_static:
                    self.gen_func(fi)
            self.moduleSafeRust.write("}\n\n")
            self.moduleSafeRust.write("impl<'a> %s + 'a {\n\n" % (t.rust_local))
            for fi in ci.methods:
                if fi.is_static:
                    self.gen_func(fi)
            self.moduleSafeRust.write("}\n\n")
        else:
            if isinstance(t, BoxedClassTypeInfo):
                self.gen_boxed_class(ci.fullname)
            if len(ci.methods) > 0:
                logging.info("Generating impl for struct %s", ci)
                self.moduleSafeRust.write("impl %s {\n\n" % (t.rust_local))
                for fi in ci.methods:
                    self.gen_func(fi)
                self.moduleSafeRust.write("}\n\n")

    def reformat_doc(self, text, comment_prefix="///"):
        text = text.strip()
        if len(text) == 0:
            return ""
        text = re.sub(r"^\s*\*$", "", text, 0, re.M)
        text = re.sub(r"^\* ", "", text, 0, re.M)
        text = text.replace("@brief", "").replace("@note", "\nNote:")
        text = text.replace("@code", "```ignore").replace("@endcode", "```\n")
        text = re.sub("^(.*?@param)", "## Parameters\n\\1", text, 1, re.M)
        text = re.sub(r".*\*\*\*\*\*", "", text, 0, re.M)
        text = re.sub("@defgroup [^ ]+ (.*)", "\\1\n\n# \\1", text)
        text = re.sub("^.*?@param ([^ ]+) (.*)", "* \\1: \\2", text, 0, re.M)
        text = re.sub("^-  (.*)", "*  \\1", text, 0, re.M)
        text = re.sub("\\\\f\[", "<div lang='latex'>", text, 0, re.M)
        text = re.sub("\\\\f\]", "</div>", text, 0, re.M)
        text = re.sub(r"\\f\$(.*?)\\f\$", "<span lang='latex'>\\1</span>", text, 0, re.M)
        # catch sequences of 4 indents and reduce them to avoid cargo test running them as code
        text = re.sub(r"^((\s{1,5})\2{3})(\S)", r"\2\3", text, 0, re.M)
        text = re.sub("^", comment_prefix + " ", text.strip(), 0, re.M) + "\n"
        return text.encode("utf-8")


def main():
    cpp_dir = sys.argv[2]
    rust_dir = sys.argv[3]
    module = sys.argv[4]
    srcfiles = sys.argv[5:]
    logging.basicConfig(filename='%s/%s.log' % (cpp_dir, module), format=None, filemode='w', level=logging.INFO)
    handler = logging.StreamHandler()
    handler.setLevel(logging.WARNING)
    logging.getLogger().addHandler(handler)
    print("Generating module '" + module + "' from headers:\n\t" + "\n\t".join(srcfiles))
    generator = RustWrapperGenerator()
    generator.gen(srcfiles, module, cpp_dir, rust_dir)


if __name__ == "__main__":
    if len(sys.argv) < 5:
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
