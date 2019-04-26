# coding: utf-8
import logging
import os.path
import re
import sys
import textwrap
from pprint import pformat
from string import Template

if sys.version_info[0] >= 3:
    from io import StringIO
else:
    from cStringIO import StringIO

#
#       EXCEPTIONS TO AUTO GENERATION
#
ManualFuncs = {
    "core": (
        ["cv.Mat.size", "Size", ["/C"], []],
    )
}

cross_modules_deps = {
    "core": (
        ["class cv.Mat", "", ["/Ghost"], []],
        ["class cv.Algorithm", "", ["/Ghost"], []],
        ["class cv.DMatch", "", ["/Ghost", "/Simple"], []],
        ["class cv.KeyPoint", "", ["/Ghost", "/Simple"], []],
        ["class cv.RotatedRect", "", ["/Ghost"], []],
        ["class cv.TermCriteria", "", ["/Ghost"], []],
        ["class cv.Range", "", ["/Ghost"], []],
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
    "cv_MatExpr_type" : "typ",
    "cv_Mat_Mat": "new",
    "cv_Mat_Mat_Mat_m": "copy",
    "cv_Mat_Mat_Mat_m_Range_ranges": "ranges",
    "cv_Mat_Mat_Mat_m_Range_rowRange_Range_colRange": "rowscols",
    "cv_Mat_Mat_Mat_m_Rect_roi": "roi",
    "cv_Mat_Mat_Size_size_int_type": "new_size",
    "cv_Mat_Mat_Size_size_int_type_Scalar_s": "new_size_with_default",
    "cv_Mat_Mat_int_rows_int_cols_int_type": "new_rows_cols",
    "cv_Mat_Mat_int_rows_int_cols_int_type_Scalar_s": "new_rows_cols_with_default",
    "cv_Mat_colRange_Range_r": "colrange",
    "cv_Mat_colRange_int_startcol_int_endcol": "colbounds",
    "cv_Mat_copyTo_Mat_m_Mat_mask": "copy_to_masked",
    "cv_Mat_create_Size_size_int_type": "-",
    "cv_Mat_create_int_rows_int_cols_int_type": "-",
    "cv_Mat_diag_Mat_d": "diag_new_mat",
    "cv_Mat_diag_int_d": "diag",
    "cv_Mat_ptr_int_i0": "ptr_row",
    "cv_Mat_ptr_int_row_int_col": "ptr_2d",
    "cv_Mat_ptr_int_i0_int_i1_int_i2": "ptr_3d",
    "cv_Mat_resize_size_t_sz": "resize",
    "cv_Mat_resize_size_t_sz_Scalar_s": "resize_with_default",
    "cv_Mat_rowRange_Range_r": "rowRange",
    "cv_Mat_rowRange_int_startrow_int_endrow": "rowbounds",
    "cv_Mat_type": "typ",
    "cv_PCA_PCA": "default",
    "cv_PCA_PCA_Mat_data_Mat_mean_int_flags_double_retainedVariance": "new_mat_variance",
    "cv_PCA_PCA_Mat_data_Mat_mean_int_flags_int_maxComponents": "new_mat_max",
    "cv_PCA_backProject_Mat_vec_Mat_result": "back_project_to",
    "cv_PCA_project_Mat_vec_Mat_result": "project_to",
    "cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_double_retainedVariance": "pca_compute_variance",
    "cv_PCACompute_Mat_data_Mat_mean_Mat_eigenvectors_int_maxComponents": "pca_compute",
    "cv_Range_Range": "default",
    "cv_Range_Range_int__start_int__end": "new",
    "cv_RotatedRect_RotatedRect": "default",
    "cv_RotatedRect_RotatedRect_Point2f_center_Size2f_size_float_angle": "new",
    "cv_RotatedRect_RotatedRect_Point2f_point1_Point2f_point2_Point2f_point3": "for_points",
    "cv_TermCriteria_TermCriteria": "default",
    "cv_TermCriteria_TermCriteria_int_type_int_maxCount_double_epsilon": "new",
    "cv_UMat_type": "typ",
    "cv_UMat_copyTo_Mat_m": "copy_to",
    "cv_UMat_copyTo_Mat_m_Mat_mask": "copy_to_masked",
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
    "cv_max_Mat_src1_double_src2_Mat_dst": "max_mat",
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
    # features2d
    "cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression": "AGAST",
    "cv_AGAST_Mat_image_VectorOfKeyPoint_keypoints_int_threshold_bool_nonmaxSuppression_int_type": "AGAST_with_type",
    "cv_BOWKMeansTrainer_cluster": "default",
    "cv_BOWKMeansTrainer_cluster_Mat_descriptors": "new",
    "cv_BOWKMeansTrainer_BOWKMeansTrainer_int_clusterCount_TermCriteria_termcrit_int_attempts_int_flags": "new_with_criteria",
    "cv_BOWImgDescriptorExtractor_compute_Mat_keypointDescriptors_Mat_imgDescriptor_VectorOfVectorOfint_pointIdxsOfClusters": "compute",
    "cv_BOWImgDescriptorExtractor_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_imgDescriptor_VectorOfVectorOfint_pointIdxsOfClusters_Mat_descriptors": "compute_desc",
    "cv_DMatch_DMatch": "default",
    "cv_DMatch_DMatch_int__queryIdx_int__trainIdx_float__distance": "new",
    "cv_DMatch_DMatch_int__queryIdx_int__trainIdx_int__imgIdx_float__distance": "new_index",
    # "cv_DescriptorExtractor_compute_Mat_image_VectorOfKeyPoint_keypoints_Mat_descriptors": "compute",
    # "cv_DescriptorExtractor_compute_VectorOfMat_images_VectorOfVectorOfKeyPoint_keypoints_VectorOfMat_descriptors": "compute_n",
    "cv_DescriptorMatcher_knnMatch_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_int_k_Mat_mask_bool_compactResult": "knn_train_matches",
    "cv_DescriptorMatcher_knnMatch_Mat_queryDescriptors_VectorOfVectorOfDMatch_matches_int_k_VectorOfMat_masks_bool_compactResult": "knn_matches",
    "cv_DescriptorMatcher_match_Mat_queryDescriptors_VectorOfDMatch_matches_VectorOfMat_masks": "matches",
    "cv_DescriptorMatcher_match_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfDMatch_matches_Mat_mask": "train_matches",
    "cv_DescriptorMatcher_radiusMatch_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfVectorOfDMatch_matches_float_maxDistance_Mat_mask_bool_compactResult": "train_radius_matches",
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
    "cv_imdecode_Mat_buf_int_flags_Mat_dst": "decode_to",
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
    # ml
    "cv_ml_ParamGrid_ParamGrid_double__minVal_double__maxVal_double__logStep": "for_range",
    # objdetect": "",
    "cv_CascadeClassifier_CascadeClassifier": "default",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_rejectLevels_VectorOfdouble_levelWeights_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize_bool_outputRejectLevels": "detect_multi_scale_levels",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_VectorOfint_numDetections_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize": "detect_multi_scale_num",
    "cv_CascadeClassifier_detectMultiScale_Mat_image_VectorOfRect_objects_double_scaleFactor_int_minNeighbors_int_flags_Size_minSize_Size_maxSize": "detect_multi_scale",
    "cv_HOGDescriptor_HOGDescriptor_HOGDescriptor_d": "copy",
    "cv_HOGDescriptor_detectMultiScale_Mat_img_VectorOfRect_foundLocations_VectorOfdouble_foundWeights_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping": "detect_multi_scale",
    "cv_HOGDescriptor_detectMultiScale_Mat_img_VectorOfRect_foundLocations_double_hitThreshold_Size_winStride_Size_padding_double_scale_double_finalThreshold_bool_useMeanshiftGrouping": "detect_multi_scale_weights",
    "cv_HOGDescriptor_detect_Mat_img_VectorOfPoint_foundLocations_VectorOfdouble_weights_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations": "detect_weights",
    "cv_HOGDescriptor_detect_Mat_img_VectorOfPoint_foundLocations_double_hitThreshold_Size_winStride_Size_padding_VectorOfPoint_searchLocations": "detect",
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
    # line_descriptor
    "cv_line_descriptor_BinaryDescriptorMatcher_match_Mat_queryDescriptors_Mat_trainDescriptors_VectorOfDMatch_matches_Mat_mask": "train_matches",
    "cv_line_descriptor_BinaryDescriptorMatcher_match_Mat_queryDescriptors_VectorOfDMatch_matches_VectorOfMat_masks": "matches",
    # utility
    "cv_getImpl_VectorOfint_impl_VectorOfString_funName": "-",
    # dnn
    "cv_dnn_<unnamed>_is_neg_int_i": "-",
    "cv_dnn_NMSBoxes_VectorOfRotatedRect_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k": "nms_boxes_rotated",
    "cv_dnn_NMSBoxes_VectorOfRect2d_bboxes_VectorOffloat_scores_float_score_threshold_float_nms_threshold_VectorOfint_indices_float_eta_int_top_k": "nms_boxes_rotated_f64",
}

class_ignore_list = (
    #core
    "CvMat", "CvArr", "CvSeq", "CvPoint.*", "CvRect", "CvTermCriteria", # have c++ equiv
    "CvSize", "CvSlice", "CvScalar",
    "Cv[A-Z].*",
    "cv::Param",
    "cv::Mat::MStep",
    "cv::Mat::MSize",
    "cv::Mutex",
    "Ipl.*",
    "BinaryFunc", "ConvertData", "ConvertScaleData",
    "cv::FileNode", "cv::FileStorage", "cv::FileNodeIterator",
    "cv::KDTree", "IndexParams", "Params", "CvAttrList",
    "cv::Exception", "cv::ErrorCallback",
    "cv::RNG.*", # maybe
    "cv::SVD",
    "cv::hal",
    "cv::TLSDataContainer",
    "NAryMatIterator",
    "cv::MatConstIterator",
    "cv::CommandLineParser",
    "cv::_InputArray", "cv::_OutputArray", "cv::_InputOutputArray",
    "cv::MatAllocator",
    "cv::SparseMat",
    "cv::AlgorithmInfo",
    #videoio
    #    "VideoWriter",
    # imgproc
    "Vertex", "QuadEdge",
    "GeneralizedHough",
    "Subdiv2D", # lots of protected stuff exported (may work now)
    # features
    "DescriptorCollection",  # nested
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

aliases_types = {
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

# Each element should be a function that takes the FuncInfo and returns a boolean indicating whether to ignore the
# FuncInfo. The FuncInfo will be ignored when the function returns true.
func_ignore_filters = (
    lambda func_info: func_info.fullname.endswith("cv::Mat::ptr") and func_info.const,
)

# regular expressions to ignore matching constant names
const_ignore_list = (
    "^CV_EXPORTS_W", "CV_MAKE_TYPE",
    "CV_IS_CONT_MAT", "CV_RNG_COEFF", "IPL_IMAGE_MAGIC_VAL",
    "CV_SET_ELEM_FREE_FLAG", "CV_FOURCC_DEFAULT",
    "CV_WHOLE_ARR", "CV_WHOLE_SEQ", "CV_PI", "CV_2PI", "CV_LOG2",
    "CV_TYPE_NAME_IMAGE",
    "CV_SUPPRESS_DEPRECATED_START",
    "CV_SUPPRESS_DEPRECATED_END",
    "__CV_BEGIN__", "__CV_END__", "__CV_EXIT__",
    "CV_IMPL_IPP", "CV_IMPL_MT", "CV_IMPL_OCL", "CV_IMPL_PLAIN",
    "CV_TRY", "CV_CATCH_ALL",
    "^CV__DEBUG_NS_",
    "UINT64_1",
    "CV_STRUCT_INITIALIZER", "CV__ENABLE_C_API_CTORS",
    "^VSX_IMPL_MULH_",
    "^CV__DNN_EXPERIMENTAL_NS_",
    "^CV_Sts",
)

#
#       TYPES MAPPING
#

primitives = {
    u"void": {u"ctype": "void", "rust_local": "()"},

    u"bool": {u"ctype": "char", u"rust_local": "bool"},

    u"char": {u"ctype": "char", u"rust_local": "i8"},
    u"schar": {u"ctype": "char", u"rust_local": "i8"},
    u"uchar": {u"ctype": "unsigned char", u"rust_local": "u8"},
    u"unsigned char": {u"ctype": "unsigned char", u"rust_local": "u8"},
    u"uchar*": {u"ctype": "unsigned char*", u"rust_local": "*mut u8"},

    u"short": {u"ctype": "short", u"rust_local": "i16"},
    u"ushort": {u"ctype": "unsigned short", u"rust_local": "u16"},

    u"int": {u"ctype": "int", u"rust_local": "i32"},
    u"uint": {u"ctype": "unsigned int", u"rust_local": "u32"},
    u"unsigned int": {u"ctype": "unsigned int", u"rust_local": "u32"},
    u"uint32_t": {u"ctype": "uint32_t", u"rust_local": "u32"},

    u"size_t": {u"ctype": "std::size_t", u"rust_local": "size_t"},

    u"int64": {u"ctype": "int64", u"rust_local": "i64"},
    u"uint64": {u"ctype": "uint64", u"rust_local": "u64"},
    u"unsigned long long": {u"ctype": "unsigned long long", u"rust_local": "u64"},
    u"int64_t": {u"ctype": "int64_t", u"rust_local": "i64"},
    u"uint64_t": {u"ctype": "uint64_t", u"rust_local": "u64"},

    u"float": {u"ctype": "float", u"rust_local": "f32"},
    u"double": {u"ctype": "double", u"rust_local": "f64"},
}

forced_trait_classes = ("cv::Algorithm", "cv::BackgroundSubtractor", "cv::dnn::Layer")

value_struct_types = {
    "Point2i": (("x", "int"), ("y", "int")),
    "Point2l": (("x", "int64"), ("y", "int64")),
    "Point2f": (("x", "float"), ("y", "float")),
    "Point2d": (("x", "double"), ("y", "double")),
    "Point": (("x", "int"), ("y", "int")),
    "Size2i": (("width", "int"), ("height", "int")),
    "Size2l": (("width", "int64"), ("height", "int64")),
    "Size2f": (("width", "float"), ("height", "float")),
    "Size2d": (("width", "double"), ("height", "double")),
    "Size": (("width", "int"), ("height", "int")),
    "Rect2i": (("x", "int"), ("y", "int"), ("width", "int"), ("height", "int")),
    "Rect2f": (("x", "float"), ("y", "float"), ("width", "float"), ("height", "float")),
    "Rect2d": (("x", "double"), ("y", "double"), ("width", "double"), ("height", "double")),
    "Rect": (("x", "int"), ("y", "int"), ("width", "int"), ("height", "int")),
    # "TermCriteria": (("type", "int"), ("maxCount", "int"), ("epsilon", "double")),
    "Scalar": (("*", "double[4]"),),
    "CvRNG": (("data", "int64"),)
}

for s in (2, 3, 4, 6):
    for t in (("uchar", "b"), ("short", "s"), ("int", "i"), ("double", "d"), ("float", "f")):
        value_struct_types["Vec%d%s" % (s,t[1])] = ("data", "%s[%d]" % (t[0],s)),

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

static_modules = ("sys", "types", "core")

#
#       TEMPLATES
#

T_CPP_MODULE = """
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

"""

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


class GeneralInfo():
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
        name = name.replace("const ", "").replace("struct ", "").replace("class ", "").replace(".", "::")
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


class ArgInfo():
    def __init__(self, gen, arg_tuple):  # [ ctype, name, def val, [mod], argno ]
        """
        :type gen: RustWrapperGenerator
        :param arg_tuple:
        """
        self.gen = gen
        self.pointer = False
        type = arg_tuple[0]
        if type.endswith("*"):
            type = type[:-1]
            self.pointer = True
        self.type = self.gen.get_type_info(type)
        self.name = arg_tuple[1]
        if not self.name:
            self.name = "unnamed_arg"
        self.defval = ""
        self.typeinfo = None
        if len(arg_tuple) > 2:
            self.defval = arg_tuple[2]
        self.out = ""
        if arg_tuple[0] in ("OutputArray", "OutputArrayOfArrays") or len(arg_tuple) > 3 and "/O" in arg_tuple[3]:
            self.out = "O"
        if arg_tuple[0] in ("InputOutputArray", "InputOutputArrayOfArrays") or len(arg_tuple) > 3 and "/IO" in arg_tuple[3]:
            self.out = "IO"

    def rsname(self):
        return camel_case_to_snake_case(reserved_rename.get(self.name, self.name))


    def __repr__(self):
        return template("ARG $ctype$p $name=$defval").substitute(ctype=self.type,
                                                                  p=" *" if self.pointer else "",
                                                                  name=self.name,
                                                                  defval="" #self.defval
                                                                )


class FuncInfo(GeneralInfo):

    KIND_FUNCTION    = "(function)"
    KIND_METHOD      = "(method)"
    KIND_CONSTRUCTOR = "(constructor)"

    def __init__(self, gen, decl, namespaces=frozenset()):  # [ funcname, return_ctype, [modifiers], [args] ]
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self._r_name = None

        if self.classname and not self.classname.startswith("operator"):
            self.ci = gen.get_class(self.classname)
            if not self.ci:
                if self.classname == "std":
                    self.is_ignored = True
                    return
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
            self.ci = None
            self.type = gen.get_type_info(decl[1])

        self.identifier = self.fullname.replace("::", "_")

        if len(decl) > 5:
            self.comment = decl[5].encode("ascii", "ignore")
        else:
            self.comment = ""

        self.args = []
        for arg in decl[3]:
            ai = ArgInfo(gen, arg)
            while any(True for x in self.args if x.name == ai.name):
                ai.name = bump_counter(ai.name)
            self.args.append(ai)
            self.identifier += "_" + ai.type.sane + "_" + ai.name

        self.const = "/C" in decl[2]
        self.static = ["", "static"]["/S" in decl[2]]

        self.fake_attrgetter = "/ATTRGETTER" in decl[2]
        self.struct_attrname = decl[6] if self.fake_attrgetter else None

        self.cname = self.cppname = self.name

        self.is_ignored = "/H" in decl[2] or "/I" in decl[2]
        if self.name.startswith("~"):
            logging.info("ignore destructor %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if self.name.startswith("operator") or self.fullname.startswith("operator "):
            logging.info("ignore %s %s in %s"%(self.kind, self.name, self.ci))
            self.is_ignored = True

        if self.is_ignored:
            return

        # register self to class or generator
        if self.kind == self.KIND_FUNCTION:
            logging.info("register %s %s (%s)"%(self.kind, self.name, self.identifier))
            gen.register_function(self)
        else:
            logging.info("register %s %s in %s (%s)"%(self.kind, self.name, self.ci, self.identifier))
            self.ci.add_method(self)

    def isconstructor(self):
        return self.kind == self.KIND_CONSTRUCTOR

    def rv_type(self):
        """
        :rtype: TypeInfo
        """
        if self.isconstructor():
            if self.ci:
                return self.gen.get_type_info(self.ci.fullname)
            else:
                return None
        else:
            return self.type

    def reason_to_skip(self):
        if self.identifier in self.gen.generated:
            return "already there"

        if self.name.startswith("operator"):
            return "can not map %s yet"%(self.name)

        for f in func_ignore_list:
            if self.fullname.endswith(f):
                return "manual ignore from list"

        if any(filter_func(self) for filter_func in func_ignore_filters):
            return "manual ignore from filter"

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
            if a.pointer and a.type.is_by_value:
                return "returning primitive by pointer is not supported (FIXME ?)"
            if a.pointer and a.type.typeid.endswith("*"):
                return "** not supported yet"
            if a.type.typeid.endswith("]"):
                return "passing raw arrays will wait (FIXME ?)"
            if a.type.typeid == "const char" and a.pointer:
                return "const char* will wait"
            if a.type.typeid == "...":
                return "variadic will have to wait"

        return None

    def gen_cpp_prelude(self):
        io = StringIO()
        io.write("// %s\n"%(self.identifier))
        io.write("// parsed: %s\n"%(self.fullname))
        io.write("// as:     %s\n"%(self))
        for a in self.args:
            ignored = ptr = ""
            if a.type.is_ignored:
                ignored = "(ignored)"
            if a.pointer:
                ptr = "(ptr)"
            io.write("// Arg %s %s %s =%s %s\n"%(a, ptr, a.type, a.defval, ignored))
        io.write("// Return value: %s\n"%(self.rv_type()))
        return io.getvalue()

    def c_name(self):
        return "cv_%s_%s" % (self.gen.module, self.identifier.replace("::", "").replace(" ", "_"))

    def r_name(self):
        if self._r_name is not None:
            return self._r_name

        if renamed_funcs.get(self.identifier):
            return renamed_funcs[self.identifier]
        name = "new" if self.isconstructor() else self.name

        return camel_case_to_snake_case(name)

    def set_r_name(self, value):
        self._r_name = value

    # "const", "mut", or None
    def mutability(self):
        if not self.ci == None and not self.isconstructor():
            return "const" if self.const else "mut"
        return ""

    def gen_rust_extern(self):
        rust_extern_rs = "cv_return_value_%s"%(self.rv_type().c_sane)

        args = []
        if not self.static and self.mutability():
            if self.ci.type_info().is_by_value:
                args.append("instance: %s"%(self.ci.type_info().rust_full))
            else:
                args.append("instance: *%s c_void"%(self.mutability()))
        for a in self.args:
            args.append(a.rsname() + ": " + a.type.rust_extern)

        return "#[doc(hidden)] pub fn %s(%s) -> %s;\n"%(self.c_name(), ", ".join(args), rust_extern_rs)

    def gen_safe_rust(self):
        args = []
        call_args = []
        cstring_args = []
        if not self.static and self.mutability() == "const":
            if self.ci.type_info().is_by_value:
                args.append("self")
                call_args.append("self")
            else:
                args.append("&self")
                call_args.append("self.as_raw_%s()"%(self.ci.type_info().rust_local))
        elif not self.static and self.mutability() == "mut":
            if self.ci.type_info().is_by_value:
                args.append("self")
                call_args.append("self")
            else:
                args.append("&mut self")
                call_args.append("self.as_raw_%s()"%(self.ci.type_info().rust_local))

        for a in self.args:
            if isinstance(a.type,StringTypeInfo):
                args.append("%s:&str"%(a.rsname()))
            elif a.type.is_by_value:
                args.append(a.rsname() + ": " + a.type.rust_full)
            elif a.out == "O" or a.out == "IO":
                args.append(a.rsname() + ": &mut " + a.type.rust_full)
            else:
                args.append(a.rsname() + ": &" + a.type.rust_full)

            if isinstance(a.type, BoxedClassTypeInfo) or a.type.is_by_ptr:
                call_args.append("%s.as_raw_%s()"%(a.rsname(), a.type.rust_local))
            elif isinstance(a.type,StringTypeInfo):
                cstring_args.append("    let %s = CString::new(%s).unwrap();\n" % (a.rsname(), a.rsname()))
                call_args.append("%s.as_ptr() as _" % (a.rsname()))
            else:
                call_args.append("%s"%(a.rsname()))

        pub = "" if self.ci and self.ci.type_info().is_trait and not self.static else "pub "

        io = StringIO()
        io.write(self.gen.reformat_doc(self.comment))
        first = True
        for a in self.args:
            if a.defval != "":
                if first:
                    io.write("///\n/// ## C++ default parameters:\n")
                io.write("/// * %s: %s\n"%(a.rsname(), a.defval))
                first = False
        io.write("%sfn %s(%s) -> Result<%s> {\n"%(pub, self.r_name(), ", ".join(args), self.rv_type().rust_full))
        io.write("// identifier: %s\n"%(self.identifier))
        io.write("  unsafe {\n")
        io.writelines(cstring_args)
        io.write("    let rv = sys::%s(%s);\n"%(self.c_name(), ", ".join(call_args)))
        io.write("    if !rv.error_msg.is_null() {\n")
        io.write("      let v = CStr::from_ptr(rv.error_msg as _).to_bytes().to_vec();\n")
        io.write("      ::libc::free(rv.error_msg as _);\n")
        io.write("      Err(Error { code: rv.error_code, message: String::from_utf8(v).unwrap() })\n")
        io.write("    } else {\n")
        if self.type.typeid == "void":
            io.write("      Ok(())\n")
        elif isinstance(self.rv_type(), StringTypeInfo):
            io.write("      let v = CStr::from_ptr(rv.result as _).to_bytes().to_vec();\n")
            io.write("      ::libc::free(rv.result as _);\n")
            io.write("      Ok(String::from_utf8(v).unwrap())\n")
        elif isinstance(self.rv_type(), SmartPtrTypeInfo):
            io.write("      Ok(%s { ptr: rv.result })\n" % (self.rv_type().rust_full))
        elif isinstance(self.rv_type(), VectorTypeInfo):
            io.write("      Ok(%s { ptr: rv.result })\n" % (self.rv_type().rust_full))
        elif isinstance(self.rv_type(), BoxedClassTypeInfo):
            io.write("      Ok(%s { ptr: rv.result })\n" % (self.rv_type().rust_full))
        else:
            io.write("      Ok(rv.result)\n")
        io.write("    }\n")
        io.write("  }\n")
        io.write("}\n\n")

        block = io.getvalue()
        if self.kind == self.KIND_FUNCTION:
            return block
        else:
            return re.sub("^(.+)$", "  \\1", block, flags=re.M)

    def __repr__(self):
        if self.kind == self.KIND_FUNCTION:
            return "%s %s"%(self.fullname, self.kind)
        else:
            return "%s %s %s . %s"%(self.fullname, self.kind, self.ci, self.name)


class ClassPropInfo():
    def __init__(self, decl):  # [f_ctype, f_name, '', '/RW']
        self.ctype = decl[0]
        self.name = decl[1]
        self.rw = "/RW" in decl[3]

    def __repr__(self):
        return template("PROP $ctype $name").substitute(ctype=self.ctype, name=self.name)


class ClassInfo(GeneralInfo):
    def __init__(self, gen, decl, namespaces):  # [ 'class/struct cname', ': base', [modlist] ]
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        self.methods = []
        self.namespaces = namespaces
        self.module = self.gen.module
        self.is_simple = self.is_ignored = self.is_ghost = False
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
        if self.classpath and gen.get_class(self.classpath).is_ignored:
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
                self.gen.get_class(base).is_trait = True

        # class props
        self.props = []
        for p in decl[3]:
            self.props.append(ClassPropInfo(p))

        self.is_ignored = self.is_ignored or self.gen.class_is_ignored(self.fullname)

        # register
        logging.info("register class %s (%s)%s%s", self.fullname, decl,
            " [ignored]" if self.is_ignored else "",
            " impl:"+",".join(self.bases) if len(self.bases) else "")
        gen.classes[self.fullname] = self
        self.add_unpack_methods()

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

    def add_unpack_methods(self):
        """
        This method is used to create methods, what allows you to access to simple types inside
        complex structures. For instance, RotatedRect items contains Point2f, Size and angle instances,
        this method creates attribute getters get_point, get_size, get_angle.
        """
        for struct_field, field_type in boxed_type_fields.get(self.classname, {}).items():
            field_decl = [
                u'{classname}.get_{struct_field}'.format(
                    classname=self.classname,
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
            self.add_method(FuncInfo(self.gen, field_decl, frozenset(self.namespaces)))

    def add_method(self, fi):
        self.methods.append(fi)

    def getAllMethods(self):
        result = []
        result.extend([fi for fi in sorted(self.methods) if fi.isconstructor()])
        result.extend([fi for fi in sorted(self.methods) if not fi.isconstructor()])
        return result

    def has_constructor(self):
        for fis in self.methods.values():
            for fi in fis:
                if fi.isconstructor():
                    return True
        return False

    def type_info(self):
        """
        :rtype: TypeInfo
        """
        return self.gen.get_type_info(self.fullname)


class ConstInfo(GeneralInfo):
    def __init__(self, gen, decl, namespaces, added_manually=False):
        GeneralInfo.__init__(self, gen, decl[0], namespaces)
        _, self.rustname = split_known_namespace(self.fullname, namespaces)
        self.rustname = self.rustname.replace("::", "_")
        self.cname = self.name.replace(".", "::")
        self.value = decl[1]
        self.added_manually = added_manually

        # register
        if self.is_ignored():
            logging.info('ignored: %s', self)
        elif not gen.get_const(self.name):
            gen.consts.append(self)

    def __repr__(self):
        return template("CONST $name=$value$manual").substitute(name=self.name,
                                                                value=self.value,
                                                                manual="(manual)" if self.added_manually else "")

    def is_ignored(self):
        for c in const_ignore_list:
            if re.match(c, self.name):
                return True
        return False

    def gen_rust(self):
        if self.value.startswith('"'):
            return "pub const %s: &'static str = %s;\n" % (self.rustname, self.value)
        elif re.match("^(-?[0-9]+|0x[0-9A-F]+)$", self.value):
            return "pub const %s: i32 = %s;\n" % (self.rustname, self.value)
        return None

    def gen_cpp_for_complex(self):
        # only use C-constant dumping for unnested const
        if len(self.fullname.split(".")) > 2:
            return ""
        elif self.fullname == "CV_VERSION":
            return """    printf("pub static %s: &'static str = \\"%%s\\";\\n", %s);\n""" % (self.rustname, self.fullname)
        elif self.fullname in ("MLN10", "RELATIVE_ERROR_FACTOR"):
            return """    printf("pub const %s: f64 = %%f;\\n", %s);\n""" % (self.rustname, self.fullname)
        else:
            return """    printf("pub const %s: i32 = 0x%%x;\\n", %s);\n""" % (self.rustname, self.fullname)


class TypeInfo:
    def __init__(self, gen, typeid):
        self.typeid = typeid
        self.gen = gen
        self.is_ignored = False
        self.is_by_ptr = False
        self.is_by_value = False
        self.is_trait = False # FIXME
        self.sane = "XX"

    def gen_wrapper(self):
        pass


class StringTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.ctype = "const char*"
        self.cpptype = "String"
        self.rust_full = "String"
        self.rust_local = "*const c_char"
        self.rust_extern = "*const c_char"
        self.sane = "String"
        self.c_sane = "char_X"

    def __str__(self):
        return "string"


class IgnoredTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_ignored = True

    def __str__(self):
        return "Ignored(%s)"%(self.typeid)


class PrimitiveTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_value = True
        self.ctype = primitives[typeid]["ctype"]
        self.cpptype = typeid
        self.rust_extern = self.rust_full = self.rust_local = primitives[typeid]["rust_local"]
        self.sane = typeid.replace(" ", "_")
        self.c_sane = self.ctype.replace(" ", "_").replace("*", "X").replace("::", "_")

    def __str__(self):
        return "Primitive(%s)"%(self.cpptype)


class EmptyTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.ctype = self.cpptype = self.rust_local = self.rust_extern = self.rust_extern = "void"


class SimpleClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)

        self.is_by_value = True
        self.ci = gen.get_class(typeid)
        if self.ci and self.ci.is_ignored:
            self.is_ignored = True
        self.cpptype = typeid
        self.rust_local = typeid.replace("cv::","").replace("::", "_")
        self.sane = self.rust_local
        if self.ci:
            self.rust_full = ("super::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
            self.ctype = "c_" + self.rust_local
            self.c_sane = self.ctype
            self.rust_extern = self.rust_full
            self.is_trait = False

    def __str__(self):
        return "%s (simple)"%(self.cpptype)


class ValueStructTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        """
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_value = True
        self.ci = gen.get_class(typeid)
        if self.ci and self.ci.is_ignored:
            self.is_ignored = True
        self.cpptype = typeid
        self.rust_local = typeid.replace("cv::","")
        self.sane = self.rust_local
        self.rust_full = "core::" + self.rust_local
        self.ctype = "c_" + self.rust_local
        self.c_sane = self.ctype
        self.rust_extern = self.rust_full
        self.is_trait = False

    def __str__(self):
        return "%s (struct)"%(self.cpptype)


class BoxedClassTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, alias):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type alias: str
        """
        TypeInfo.__init__(self, gen, typeid)
        self.ci = gen.get_class(typeid)
        self.cpptype = self.ci.fullname
        self.rust_extern = "*mut c_void"
        _, self.rust_local = split_known_namespace(typeid, gen.namespaces)
        self.rust_local = self.rust_local.replace("::", "_")
        self.rust_full = ("super::" if self.ci.module not in static_modules else "") + self.ci.module + "::" + self.rust_local
        self.is_by_ptr = True
        self.is_trait = typeid in forced_trait_classes or self.ci.is_trait
        self.ctype = "void*"
        self.c_sane = "void_X"
        self.is_ignored = self.ci.is_ignored
        if alias:
            self.sane = alias
        else:
            self.sane = self.ci.name

    def __str__(self):
        return "%s (boxed)"%(self.typeid)


class VectorTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = inner.is_ignored
        if not self.is_ignored:
            self.ctype = "void*"
            self.c_sane = "void_X"
            self.inner_cpptype = inner.cpptype
            if isinstance(self.inner, SmartPtrTypeInfo):
                self.outer_cpptype = "std::vector<%s>" % (self.inner.outer_cpptype)
                self.inner_outer_cpptype = self.inner.outer_cpptype
            self.cpptype = "std::vector<%s>" % (inner.cpptype)
            self.sane = self.rust_local = "VectorOf"+inner.sane
            self.rust_full = "types::" + self.rust_local
            self.rust_extern = "*mut c_void"
            self.inner_rust_full = inner.rust_full

    def gen_wrapper(self):
        with open(self.gen.rust_dir + "/" + self.sane + ".type.rs", "w") as f:
            if self.inner.typeid != "bool":
                f.write(template("""
                extern "C" {
                    #[doc(hidden)] fn cv_${sane}_data(ptr:*mut c_void) -> *mut c_void;
                }
                """).substitute(self.__dict__))
            f.write(template("""
                extern "C" {
                   #[doc(hidden)] fn cv_new_$sane() -> *mut c_void;
                   #[doc(hidden)] fn cv_delete_$sane(ptr:*mut c_void) -> ();
                   #[doc(hidden)] fn cv_push_$sane(ptr:*mut c_void, ptr2: *const c_void) -> ();
                   #[doc(hidden)] fn cv_${sane}_len(ptr:*mut c_void) -> i32;
                   #[doc(hidden)] fn cv_${sane}_get(ptr:*mut c_void, index: i32) -> *mut c_void;
                }
                #[allow(dead_code)] pub struct $rust_local {
                    #[doc(hidden)] pub ptr: *mut c_void
                }
                impl $rust_full {
                    pub fn new() -> $rust_local {
                        unsafe { return $rust_local { ptr: cv_new_$sane() } };
                    }
                    pub fn len(&self) -> i32 {
                        unsafe { return cv_${sane}_len(self.ptr); }
                    }
                    #[doc(hidden)] pub unsafe fn as_raw_$rust_local(&self) -> *mut c_void {
                        self.ptr
                    }

                }
                impl Drop for $rust_local {
                    fn drop(&mut self) {
                        unsafe { cv_delete_$sane(self.ptr) };
                    }
                }
                """).substitute(self.__dict__))
        with open(self.gen.rust_dir + "/" + self.sane + ".type.rs", "a") as f:
            if isinstance(self.inner, BoxedClassTypeInfo) or self.inner.is_by_ptr:
                f.write(template("""
                    // BoxedClassTypeInfo
                    impl $rust_full {
                        pub fn push(&mut self, val: $inner_rust_full) {
                            unsafe { cv_push_$sane(self.ptr, val.ptr) }
                        }
                        pub fn get(&self, index: i32) -> $inner_rust_full {
                            let raw_pointer: *mut c_void = unsafe {cv_${sane}_get(self.ptr, index)};
                            return $inner_rust_full{ ptr: raw_pointer}
                        }
                        pub fn to_vec(&self) -> Vec<$inner_rust_full> {
                            (0..self.len()).map(|x| self.get(x)).collect()
                        }
                    }
                    """).substitute(self.__dict__))
            else:
                f.write(template("""
                    impl $rust_full {
                        pub fn push(&mut self, val: $inner_rust_full) {
                            unsafe { cv_push_$sane(self.ptr, &val as *const _ as *const _) }
                        }
                        pub fn get(&self, index: i32) -> &mut $inner_rust_full {
                            let raw_pointer: *mut c_void = unsafe {cv_${sane}_get(self.ptr, index)};
                            let mut_data: &mut $inner_rust_full = unsafe { &mut *(raw_pointer as *mut $inner_rust_full )};
                            return mut_data
                        }
                    }
                    """).substitute(self.__dict__))
                if self.inner.typeid != "bool":
                    f.write(template("""
                       impl ::std::ops::Deref for $rust_local {
                           type Target = [$inner_rust_full];
                           fn deref(&self) -> &[$inner_rust_full] {
                               unsafe {
                                   let length = cv_${sane}_len(self.ptr) as usize;
                                   let data = cv_${sane}_data(self.ptr);
                                   ::std::slice::from_raw_parts(::std::mem::transmute(data), length)
                               }
                           }
                       }
                   """).substitute(self.__dict__))
        with open(self.gen.cpp_dir + "/" + self.sane + ".type.cpp", "w") as f:
            if isinstance(self.inner, SmartPtrTypeInfo):
                externs = template("""
                    void* cv_new_$sane() { return new $outer_cpptype(); }
                    void cv_delete_$sane(void* ptr) { delete (($outer_cpptype*) ptr); }
                    void cv_push_$sane(void* ptr, void* ptr2) {
                        $inner_outer_cpptype* val = ($inner_outer_cpptype*)ptr2;
                        (($outer_cpptype*) ptr)->push_back(*val);
                    }
                    $inner_outer_cpptype* cv_${sane}_front_item(void* ptr) {
                        $inner_outer_cpptype val = (($outer_cpptype*) ptr)->front();
                        return new $inner_outer_cpptype(val);
                    }
                    $inner_outer_cpptype* cv_${sane}_back_item(void* ptr) {
                        $inner_outer_cpptype val = (($outer_cpptype*) ptr)->back();
                        return new $inner_outer_cpptype(val);
                    }
                    int cv_${sane}_len(void* ptr) { return (($outer_cpptype*) ptr)->size(); }
                """).substitute(self.__dict__)
            else:
                externs = template("""
                    void* cv_new_$sane() { return new $cpptype(); }
                    void cv_delete_$sane(void* ptr) { delete (($cpptype*) ptr); }
                    void cv_push_$sane(void* ptr, void* ptr2) {
                        $inner_cpptype* val = ($inner_cpptype*)ptr2;
                        (($cpptype*) ptr)->push_back(*val);
                    }
                    $inner_cpptype* cv_${sane}_front_item(void* ptr) {
                        $inner_cpptype val = (($cpptype*) ptr)->front();
                        return new $inner_cpptype(val);
                    }
                    $inner_cpptype* cv_${sane}_back_item(void* ptr) {
                        $inner_cpptype val = (($cpptype*) ptr)->back();
                        return new $inner_cpptype(val);
                    }
                    int cv_${sane}_len(void* ptr) { return (($cpptype*) ptr)->size(); }
                    """).substitute(self.__dict__)
            if self.inner.typeid != "bool":
                if isinstance(self.inner, SmartPtrTypeInfo):
                    externs += template("""
                        $inner_outer_cpptype* cv_${sane}_get(void* ptr, int index) {
                            $inner_outer_cpptype val = (($outer_cpptype*) ptr)->data()[index];
                            return new $inner_outer_cpptype(val);
                        }
                        $ctype* cv_${sane}_data(void* ptr) {
                            return ($ctype*) ((($outer_cpptype*) ptr)->data());
                        }
                    """).substitute(self.__dict__)
                else:
                    externs += template("""
                        $inner_cpptype* cv_${sane}_get(void* ptr, int index) {
                            $inner_cpptype val = (($cpptype*) ptr)->data()[index];
                            return new $inner_cpptype(val);
                        }
                        $ctype* cv_${sane}_data(void* ptr) {
                            return ($ctype*) ((($cpptype*) ptr)->data());
                        }
                    """).substitute(self.__dict__)
            else:
                externs += template("""
                    $inner_cpptype* cv_${sane}_get(void* ptr, int index) {
                        $inner_cpptype val = (*(($cpptype*) ptr))[index];
                        return new $inner_cpptype(val);
                    }
                """).substitute(self.__dict__)
            f.write(template(T_CPP_MODULE).substitute(code=externs, includes=""))

    def __str__(self):
        return "Vector[%s]"%(self.inner)


class SmartPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        """
        :type gen: RustWrapperGenerator
        :type typeid: str
        :type inner: TypeInfo
        """
        TypeInfo.__init__(self, gen, typeid)
        self.is_by_ptr = True
        self.inner = inner
        self.is_ignored = self.inner.is_ignored
        if not self.is_ignored:
            self.ctype = "void*"
            self.c_sane = "void_X"
            self.rust_extern = "*mut c_void"
            self.cpptype = self.inner.cpptype
            self.outer_cpptype = "Ptr<"+self.inner.cpptype+">"
            self.rust_local = self.sane = "PtrOf" + inner.sane
            self.rust_full = "types::" + self.rust_local
            self.inner_rust_full = inner.rust_full
            self.inner_local = inner.rust_local

    def gen_wrapper(self):
        with open(self.gen.rust_dir + "/" + self.rust_local + ".type.rs", "w") as f:
            f.write(template("""
                #[allow(dead_code)]
                pub struct $rust_local {
                    #[doc(hidden)] pub ptr: *mut c_void
                }
                extern "C" {
                    #[doc(hidden)] fn cv_${sane}_get(ptr:*mut c_void) -> *mut c_void;
                    #[doc(hidden)] fn cv_delete_$sane(ptr:*mut c_void);
                }
                impl $rust_full {
                    #[doc(hidden)] pub unsafe fn as_raw_$rust_local(&self) -> *mut c_void {
                        self.ptr
                    }
                }
                impl Drop for $rust_full {
                    fn drop(&mut self) {
                        unsafe { cv_delete_$sane(self.ptr) };
                    }
                }
                """).substitute(self.__dict__))
            if not isinstance(self.inner, PrimitiveTypeInfo) and self.inner.ci.is_trait:
                bases = self.gen.all_bases(self.inner.ci.name).union(set((self.inner.typeid,)))
                for base in bases:
                    cibase = self.gen.get_type_info(base)
                    if not isinstance(cibase, UnknownTypeInfo):
                        f.write(template("""
                            impl $base_full for $rust_name {
                                #[doc(hidden)] fn as_raw_$base_local(&self) -> *mut c_void { 
                                    unsafe { cv_${sane}_get(self.ptr) }
                                }
                            }
                        """).substitute(rust_name=self.rust_local, base_local=cibase.rust_local, base_full=cibase.rust_full, sane=self.sane))
        with open(self.gen.cpp_dir + "/" + self.sane + ".type.cpp", "w") as f:
            code = template("""
                void* cv_${sane}_get(void* ptr) {
                    return (($outer_cpptype*)ptr)->get();
                }
                void  cv_delete_$sane(void* ptr) {
                    delete ($outer_cpptype*) ptr;
                }
            """).substitute(self.__dict__)
            f.write(template(T_CPP_MODULE).substitute(code=code, includes=""))

    def __str__(self):
        return "SmartPtr[%s]"%(self.inner)


class RawPtrTypeInfo(TypeInfo):
    def __init__(self, gen, typeid, inner):
        TypeInfo.__init__(self,gen,typeid)
        self.inner = inner
        self.is_ignored = True
        self.sane = inner.sane

    def __str__(self):
        return "RawPtr[%s]"%(self.inner)


class UnknownTypeInfo(TypeInfo):
    def __init__(self, gen, typeid):
        TypeInfo.__init__(self,gen,typeid)
        self.is_ignored = True
        logging.info("Registering an unknown type: %s", typeid)

    def __str__(self):
        return "Unknown[%s]"%(self.typeid)


#class ReferenceTypeInfo(TypeInfo):
#    def __init__(self, gen, typeid, inner):
#        TypeInfo.__init__(self,gen,typeid)
#        self.inner = inner
#        self.is_ignored = self.inner.is_ignored
#        if not self.inner.is_ignored:
#            self.cpptype = self.inner.cpptype
#            self.ctype = self.inner.ctype
#            self.rust_local = self.inner.rust_local
#            self.rust_extern = self.inner.rust_extern
#            self.rust_extern = self.inner.rust_extern
#            self.is_by_value = self.inner.is_by_value
#
#    def __str__(self):
#        return "Ref[%s]"%(self.inner)


def parse_type(gen, typeid):
    """
    :type gen: RustWrapperGenerator
    :type typeid: str
    :rtype: TypeInfo
    """
    typeid = typeid.strip()\
        .replace("const ", "")\
        .replace("..", ".")
    if typeid == "":
        typeid = "void"
    # if typeid.endswith("&"):
    #     return ReferenceTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    if typeid.endswith("&"):
        typeid = typeid[0:-1].strip()
    if typeid in primitives:
        return PrimitiveTypeInfo(gen, typeid)
    elif typeid.endswith("*"):
        return RawPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-1]))
    elif typeid.endswith("[]"):
        return RawPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[0:-2]))
    elif typeid == "string" or typeid == "String":
        return StringTypeInfo(gen,typeid)
    elif typeid == "":
        raise NameError("empty type detected")
    elif typeid.startswith("Ptr<"):
        return SmartPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[4:-1]))
#        return RawPtrTypeInfo(gen, typeid, gen.get_type_info(typeid[4:-1]))
    elif typeid.startswith("vector<"):
        inner = gen.get_type_info(typeid[7:-1])
        if not inner:
            raise NameError("inner type `%s' not found"%(typeid[7:-1]))
        return VectorTypeInfo(gen, typeid, inner)
    elif typeid.startswith("std::vector<"):
        inner = gen.get_type_info(typeid[12:-1])
        if not inner:
            raise NameError("inner type `%s' not found"%(typeid[12:-1]))
        return VectorTypeInfo(gen, typeid, inner)
    elif gen.get_value_struct(typeid):
        return ValueStructTypeInfo(gen, gen.get_value_struct(typeid))
    else:
        ci = gen.get_class(typeid)
        if ci and not ci.is_ignored:
            if ci.is_simple:
                return SimpleClassTypeInfo(gen, ci.fullname)
            else:
                return BoxedClassTypeInfo(gen, ci.fullname, None)
        actual = aliases_types.get(typeid)
        if actual:
            ci = gen.get_class(actual)
            if ci:
                if ci.is_simple:
                    return SimpleClassTypeInfo(gen, ci.fullname)
                else:
                    return BoxedClassTypeInfo(gen, ci.fullname, None)
            return parse_type(gen, actual)
    return UnknownTypeInfo(gen, typeid)

#
#       GENERATOR
#


def template(text):
    return Template(textwrap.dedent(text))


class RustWrapperGenerator(object):
    def __init__(self):
        self.cpp_dir = ""
        self.rust_dir = ""
        self.module = ""
        self.classes = {}
        self.functions = []
        self.ported_func_list = []
        self.skipped_func_list = []
        self.consts = []
        self.type_infos = {}
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
            if c.fullname.endswith("::"+classname):
                return c
        return None

    def get_value_struct(self, classname):
        c = value_struct_types.get(classname)
        if c:
            return classname
        for c in value_struct_types.keys():
            if c.endswith("::" + classname):
                return c
        return None

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
        for c in self.consts:
            if c.cname == name:
                return c
        return None

    def add_decl(self, decl):
        if decl[0] == "cv.String.String" or decl[0] == 'cv.Exception.~Exception':
            return
        if decl[0] == "cv.Algorithm":
            decl[0] = "cv.Algorithm.Algorithm"
        name = decl[0]
        if name.startswith("struct") or name.startswith("class"):
            ClassInfo(self, decl, frozenset(self.namespaces))
        elif name.startswith("const"):
            ConstInfo(self, decl, frozenset(self.namespaces))
        else:
            FuncInfo(self, decl, frozenset(self.namespaces))

    def register_function(self, f):
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
        self.module = module
        includes = []

        parser = hdr_parser.CppHeaderParser()
        self.namespaces = set(x for x in parser.namespaces)
        self.namespaces.add("cv")

        for m in cross_modules_deps:
            self.module = m
            for d in cross_modules_deps[m]:
                self.add_decl(d)

        self.module = module

        for hdr in srcfiles:
            decls = parser.parse(hdr, False)
            self.namespaces = set(str(x.replace(".", "::")) for x in parser.namespaces)
            logging.info("\n\n=============== Header: %s ================\n\n", hdr)
            logging.info("Namespaces: %s", parser.namespaces)
            logging.info("Comment: %s", parser.comment)
            includes.append('#include "' + hdr + '"')
            for decl in decls:
                logging.info("\n--- Incoming ---\n%s", pformat(decl, 4))
                self.add_decl(decl)

        if module in ManualFuncs:
            for decl in ManualFuncs[self.module]:
                logging.info("\n--- Manual ---\n%s", pformat(decl, 4))
                self.add_decl(decl)


        logging.info("\n\n===== Generating... =====")
        self.moduleCppTypes = StringIO()
        self.moduleCppCode = StringIO()
        self.moduleCppConsts = StringIO()
        self.moduleSafeRust = StringIO()
        self.moduleRustExterns = StringIO()

        self.moduleSafeRust.write('//! <script type="text/javascript" src="http://latex.codecogs.com/latexit.js"></script>\n'.encode("utf-8"))
        self.moduleSafeRust.write(self.reformat_doc(parser.comment).encode("utf-8").replace("///", "//!"))

        self.moduleSafeRust.write(template("""
            use libc::{c_void, c_char, size_t};
            use std::ffi::{CStr, CString};
            use crate::{core, sys, types};
            use crate::{Error, Result};
        """).substitute())
        for co in sorted(self.consts, key=lambda c: c.rustname):
            rust = co.gen_rust()
            if rust:
                self.moduleSafeRust.write(rust)
            else:
                self.moduleCppConsts.write(co.gen_cpp_for_complex())

        self.moduleSafeRust.write("\n")

        for ci in sorted(self.classes.values(), key=lambda ci: ci.fullname):
            if ci.classpath:
                self.gen_nested_class_decl(ci)

        if module == "core":
            for c in sorted(value_struct_types, key=lambda c: c[0]):
                self.gen_value_struct(c)

        for t in self.type_infos.values():
            if not t.is_ignored:
                t.gen_wrapper()

        for c in self.classes.values():
            if c.is_simple and not c.is_ignored and not c.is_ghost:
                self.gen_simple_class(c)

        for fi in sorted(self.functions, key=lambda fi: fi.identifier):
            if not fi.is_ignored:
                self.gen_func(fi)

#        if module in forced_boxed_classes:
#            for cb in sorted(forced_boxed_classes[module]):
#                self.gen_boxed_class(cb)

        for ci in sorted(self.classes.values(), key=lambda ci:ci.fullname):
            if not ci.is_ignored and not ci.is_ghost:
                self.gen_class(ci)

        with open(cpp_dir + "/types.h", "a") as f:
            f.write(self.moduleCppTypes.getvalue())

        with open(cpp_dir + "/" + self.module + ".consts.cpp", "w") as f:
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
            f.write(template(T_CPP_MODULE).substitute(m = module, M = module.upper(), code = self.moduleCppCode.getvalue(), includes = "\n".join(includes), namespaces=namespaces))

        with open(rust_dir + "/%s.externs.rs" % (module), "w") as f:
            f.write("extern \"C\" {\n")
            f.write(self.moduleRustExterns.getvalue())
            f.write("}\n")

        with open(rust_dir + "/" + module + ".rs", "w") as f:
            f.write(self.moduleSafeRust.getvalue())

        with open(cpp_dir + "/" + module + ".txt", "w") as f:
            f.write(self.makeReport())

    def makeReport(self):
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

        self.moduleCppCode.write(fi.gen_cpp_prelude())

        decl_c_args = "\n        "
        call_cpp_args = ""
        if fi.ci is not None and not fi.isconstructor() and not fi.static:
            decl_c_args += fi.ci.type_info().ctype + " instance"
        for a in fi.args:

            if not decl_c_args.strip() == "":
                decl_c_args+=",\n        "
            if not call_cpp_args == "":
                call_cpp_args += ", "

            rw = a.out == "O" or a.out == "IO"

            arg_decl_star = not isinstance(a.type, BoxedClassTypeInfo) and rw
            if isinstance(a.type, StringTypeInfo):
                decl_c_args += "const char *" + a.name
            elif arg_decl_star:
                decl_c_args += a.type.ctype + " *" + a.name
            else:
                decl_c_args += a.type.ctype + " " + a.name

            if isinstance(a.type, SmartPtrTypeInfo):
                call_cpp_args += "reinterpret_cast<" + a.type.cpptype + " *>(" +  a.name + ")"
            elif a.type.is_by_ptr:
                if a.pointer:
                    call_cpp_args += "((%s*)%s)"%(a.type.cpptype.replace("&",""), a.name)
                elif isinstance(a.type, VectorTypeInfo) and isinstance(a.type.inner, SmartPtrTypeInfo):
                    call_cpp_args += "*((%s*)%s)"%(a.type.outer_cpptype.replace("&",""), a.name)
                else:
                    call_cpp_args += "*((%s*)%s)"%(a.type.cpptype.replace("&",""), a.name)
            elif isinstance(a.type, StringTypeInfo):
                call_cpp_args += "%s(%s)" % (a.type.cpptype, a.name)
            elif a.type.is_by_value:
                if arg_decl_star and a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.type.cpptype + "*>(" +  a.name + ")"
                elif arg_decl_star and not a.pointer:
                    call_cpp_args += "*reinterpret_cast<" + a.type.cpptype + "*>(" +  a.name + ")"
                elif a.pointer:
                    call_cpp_args += "reinterpret_cast<" + a.type.cpptype + "*>(&" +  a.name + ")"
                else:
                    call_cpp_args += "*reinterpret_cast<" + a.type.cpptype + "*>(&" +  a.name + ")"
            else:
                if arg_decl_star and a.pointer:
                    call_cpp_args += a.name
                elif not arg_decl_star and not a.pointer:
                    call_cpp_args += a.name
                else:
                    call_cpp_args += "*" + a.name


        # C function prototype
        self.moduleCppCode.write("struct cv_return_value_%s %s(%s) {\n" % (fi.rv_type().c_sane, fi.c_name(), decl_c_args))

        self.moduleCppCode.write("  try {\n")
        # cpp method call with prefix
        if fi.ci == None and (fi.cppname.startswith("cv") or fi.cppname.startswith("CV")):
            call_name = fi.cppname
        elif fi.ci == None or fi.static:
            call_name = fi.fullname.replace(".", "::")
        elif fi.isconstructor() and isinstance(fi.ci.type_info(), BoxedClassTypeInfo):
            call_name = fi.ci.fullname
        elif fi.cppname == "()":
            call_name = "(*((%s*) instance))" % (fi.ci.fullname)
        elif fi.fake_attrgetter:
            if isinstance(fi.type, PrimitiveTypeInfo):
                call_name = "(({typename}*) instance)->{attrname}".format(
                    typename=fi.classname,
                    attrname=fi.struct_attrname,
                )
            else:
                call_name = "({typeid}) (({typename}*) instance)->{attrname}".format(
                    typeid=fi.type.typeid,
                    typename=fi.classname,
                    attrname=fi.struct_attrname,
                )

        elif isinstance(self.get_type_info(fi.ci.name), BoxedClassTypeInfo):
            call_name = "((%s*) instance)->%s" % (fi.ci.fullname, fi.cppname)
        else:
            call_name = "((%s*) &instance)->%s" % (fi.ci.fullname, fi.cppname)

        # actual call
        if fi.type.ctype == "void":
            self.moduleCppCode.write("    %s(%s);\n"%(call_name, call_cpp_args))
        elif fi.isconstructor() and (isinstance(fi.rv_type(), BoxedClassTypeInfo)):
            self.moduleCppCode.write("    %s* cpp_return_value = new %s(%s);\n" % (fi.rv_type().cpptype, call_name, call_cpp_args))
        elif isinstance(fi.rv_type(), SmartPtrTypeInfo):
            if fi.fake_attrgetter:
                self.moduleCppCode.write("    Ptr<%s> *cpp_return_value = new Ptr<%s>;\n" % (fi.rv_type().cpptype, call_name))
            else:
                self.moduleCppCode.write(
                    "    Ptr<%s> *cpp_return_value = new Ptr<%s>(%s(%s));\n" % (fi.rv_type().cpptype, fi.rv_type().cpptype, call_name, call_cpp_args))
        elif fi.isconstructor() and call_cpp_args != "":
            self.moduleCppCode.write("    %s cpp_return_value(%s);\n" % (fi.rv_type().cpptype, call_cpp_args))
        elif fi.isconstructor():
            self.moduleCppCode.write("    %s cpp_return_value;\n" % (fi.rv_type().cpptype))
        else:
            if fi.fake_attrgetter:
                self.moduleCppCode.write("    %s cpp_return_value = %s;\n" % (fi.rv_type().cpptype, call_name))
            else:
                self.moduleCppCode.write("    %s cpp_return_value = %s(%s);\n" % (fi.rv_type().cpptype, call_name, call_cpp_args))

        self.gen_c_return_value_type(fi.rv_type())

        # return value
        if fi.type.ctype == "void":
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL };\n")
        elif isinstance(fi.rv_type(), StringTypeInfo):
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, strdup(cpp_return_value.c_str()) };")
        elif isinstance(fi.rv_type(), BoxedClassTypeInfo) and not fi.isconstructor():
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, new %s(cpp_return_value) };\n" % (fi.rv_type().cpptype))
        elif isinstance(fi.rv_type(), BoxedClassTypeInfo) and fi.isconstructor():
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, cpp_return_value };\n")
        elif isinstance(fi.rv_type(), SmartPtrTypeInfo):
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, cpp_return_value };\n")
        elif fi.rv_type().is_by_value:
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, *reinterpret_cast<%s*>(&cpp_return_value) };\n"%(fi.rv_type().ctype))
        elif isinstance(fi.rv_type(), VectorTypeInfo):
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, (void*) new %s(cpp_return_value) };\n" % (fi.rv_type().cpptype))
        else: # prim
            self.moduleCppCode.write("    return { Error::Code::StsOk, NULL, cpp_return_value };\n")

        self.moduleCppCode.write("  } catch (cv::Exception& e) {\n")
        self.moduleCppCode.write("    char* msg = strdup(e.what());\n")
        if fi.type.ctype == "void":
            self.moduleCppCode.write("    return { e.code, msg };\n")
        else:
            self.moduleCppCode.write("    struct cv_return_value_%s ret;\n" % (fi.rv_type().c_sane))
            self.moduleCppCode.write("    memset(&ret, 0x00, sizeof(ret));\n")
            self.moduleCppCode.write("    ret.error_code = e.code;\n")
            self.moduleCppCode.write("    ret.error_msg = msg;\n")
            self.moduleCppCode.write("    return ret;\n")
        self.moduleCppCode.write("  } catch (...) {\n")
        self.moduleCppCode.write("    char* msg = strdup(\"unspecified error in OpenCV guts\");\n")
        if fi.type.ctype == "void":
            self.moduleCppCode.write("    return { -99999, msg };\n")
        else:
            self.moduleCppCode.write("    struct cv_return_value_%s ret;\n" % (fi.rv_type().c_sane))
            self.moduleCppCode.write("    memset(&ret, 0x00, sizeof(ret));\n")
            self.moduleCppCode.write("    ret.error_code = -99999;\n")
            self.moduleCppCode.write("    ret.error_msg = msg;\n")
            self.moduleCppCode.write("    return ret;\n")
        self.moduleCppCode.write("  }\n")

        self.moduleCppCode.write("}\n\n")

        # rust's extern C
        self.moduleRustExterns.write(fi.gen_rust_extern())

        # Here we filter and rename functions with duplicate names.
        # If duplicate functions have different call arguments
        # we generate new name for duplicate function, to allow
        # to call it from rust code.
        # If duplicate functions have the same call arguments, we skip duplicate function.
        rust_func_name = fi.r_name()
        classname = "" if fi.kind == fi.KIND_FUNCTION else fi.classname
        while classname + '::' + rust_func_name in self.func_names:
            rust_func_name = bump_counter(rust_func_name)
        fi.set_r_name(rust_func_name)

        # rust safe wrapper
        self.moduleSafeRust.write(fi.gen_safe_rust())
        self.func_names.add(classname + '::' + fi.r_name())

    def gen_value_struct_field(self, name, typ, is_simple_struct=False):
        if name == "*":
            name = "data"
        rsname = reserved_rename.get(name, name)
        visibility = "" if rsname == "__rust_private" else "pub "
        if "[" in typ:
            bracket = typ.index("[")
            cppt = typ[:bracket]
            ct = self.get_type_info(cppt).ctype
            size = typ[bracket+1:-1]
            rst = self.get_type_info(cppt).rust_full
            self.moduleCppTypes.write("    %s %s[%s];\n"%(ct, name, size))
            if is_simple_struct:
                self.moduleSafeRust.write("%s[%s; %s], " % (visibility, rst, size))
            else:
                self.moduleSafeRust.write("    %s%s: [%s; %s],\n" % (visibility, rsname, rst, size))
        else:
            typ = self.get_type_info(typ)
            self.moduleCppTypes.write("    %s %s;\n"%(typ.ctype, name))
            if is_simple_struct:
                self.moduleSafeRust.write("%s %s, " % (visibility, typ.rust_full))
            else:
                self.moduleSafeRust.write("    %s%s: %s,\n"%(visibility, rsname, typ.rust_full))

    def gen_value_struct(self, c):
        self.moduleCppTypes.write("typedef struct c_%s {\n"%(c.replace("::","_")))
        self.moduleSafeRust.write("// manually defined value struct %s\n" % (c.split("::")[-1]))
        self.moduleSafeRust.write("#[repr(C)]\n#[derive(Copy,Clone,Debug,PartialEq)]\npub struct %s" % (c.split("::")[-1]))
        is_simple_struct = len(value_struct_types[c]) == 1 and value_struct_types[c][0][0] == "*"
        if is_simple_struct:
	        self.moduleSafeRust.write(" (")
        else:
	        self.moduleSafeRust.write(" {\n")
        for field in value_struct_types[c]:
	        self.gen_value_struct_field(field[0], field[1], is_simple_struct)
        self.moduleCppTypes.write("} c_%s;\n\n" % (c.replace("::", "_")))
        if is_simple_struct:
	        self.moduleSafeRust.write(");\n\n")
        else:
	        self.moduleSafeRust.write("}\n\n")

    def gen_simple_class(self,ci):
        self.moduleSafeRust.write(self.reformat_doc(ci.comment))
        self.moduleSafeRust.write("#[repr(C)]\n#[derive(Copy,Clone,Debug,PartialEq)]\npub struct %s {\n" % (ci.type_info().rust_local))
        self.moduleCppTypes.write("typedef struct %s {\n" % (ci.type_info().c_sane))
        if len(ci.props) > 0:
	         for p in ci.props:
		          self.gen_value_struct_field(p.name, p.ctype)
        else:
	         self.gen_value_struct_field("__rust_private", "unsigned char[0]")
        self.moduleSafeRust.write("}\n\n")
        self.moduleCppTypes.write("} %s;\n\n" % (ci.type_info().c_sane))

    def gen_c_return_value_type(self, typ):
        with open(self.cpp_dir + "/cv_return_value_" + typ.c_sane + ".type.h", "w") as f:
            if typ.ctype == "void":
                f.write(template("""
                    // $typeid
                    struct cv_return_value_$c_sane {
                        int error_code;
                        char* error_msg;
                    };
                    """).substitute(typ.__dict__))
            else:
                f.write(template("""
                    // $typeid
                    struct cv_return_value_$c_sane {
                        int error_code;
                        char* error_msg;
                        $ctype result;
                    };
                    """).substitute(typ.__dict__))
        with open(self.rust_dir + "/cv_return_value_" + typ.c_sane + ".rv.rs", "w") as f:
            if typ.ctype == "void":
                f.write(template("""
                    // $typeid
                    #[repr(C)]
                    pub struct cv_return_value_void {
                        pub error_code: i32,
                        pub error_msg: *const c_char,
                    }
                    """).substitute(typ.__dict__))
            else:
                f.write(template("""
                    // $typeid
                    #[repr(C)]
                    pub struct cv_return_value_$c_sane {
                        pub error_code: i32,
                        pub error_msg: *const c_char,
                        pub result: $rust_extern
                    }
                    """).substitute(typ.__dict__))

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

        self.moduleRustExterns.write("pub fn cv_delete_%s(ptr : *mut c_void);\n" % (typ.sane))

        self.moduleSafeRust.write("// boxed class %s\n"%(typ.typeid))
        self.moduleSafeRust.write(self.reformat_doc(ci.comment))
        self.moduleSafeRust.write(template("""
            #[allow(dead_code)]
            pub struct $rust_local {
                #[doc(hidden)] pub ptr: *mut c_void
            }
            impl Drop for $rust_full {
                fn drop(&mut self) {
                    unsafe { sys::cv_delete_$sane(self.ptr) };
                }
            }
            impl $rust_full {
                #[doc(hidden)] pub fn as_raw_$rust_local(&self) -> *mut c_void { self.ptr }
            }
            """).substitute(typ.__dict__))

        bases = self.all_bases(name)
        for base in bases:
            cibase = self.get_class(base)
            if cibase is not None:
                cibase = cibase.type_info()
                self.moduleSafeRust.write(template("""
                    impl $base_full for $rust_name {
                        #[doc(hidden)] fn as_raw_$base_local(&self) -> *mut c_void { self.ptr }
                    }
                """).substitute(rust_name=typ.rust_local, base_local=cibase.rust_local, base_full=cibase.rust_full))

    def gen_nested_class_decl(self, ci):
        pass
        # self.moduleCppCode.write("class %s;\n"%(ci.nested_cname))

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
        if ci.is_ignored:
            logging.info("Manual ignore class %s", ci)
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
            self.moduleSafeRust.write("  #[doc(hidden)] fn as_raw_%s(&self) -> *mut c_void;\n" % (t.rust_local))
            for fi in ci.methods:
                if not fi.static:
                    self.gen_func(fi)
            self.moduleSafeRust.write("}\n")
            self.moduleSafeRust.write("impl<'a> %s + 'a {\n\n" % (t.rust_local))
            for fi in ci.methods:
                if fi.static:
                    self.gen_func(fi)
            self.moduleSafeRust.write("}\n\n")
        else:
            if isinstance(t, BoxedClassTypeInfo):
                self.gen_boxed_class(ci.fullname)
            logging.info("Generating impl for struct %s", ci)
            self.moduleSafeRust.write("impl %s {\n\n" % (t.rust_local))
            for fi in ci.methods:
                self.gen_func(fi)
            self.moduleSafeRust.write("}\n")

    def reformat_doc(self, text):
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
        text = re.sub("^", "/// ", text.strip(), 0, re.M) + "\n"
        return text


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
