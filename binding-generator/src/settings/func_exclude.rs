use std::collections::HashSet;

use once_cell::sync::Lazy;

/// map of functions to exclude, value is Func.identifier()
pub static FUNC_EXCLUDE: Lazy<HashSet<&str>> = Lazy::new(|| {
	HashSet::from([
		// ### core ###
		"cv_AsyncArray__getImpl_const",
		"cv_MatExpr_propOp_const",               // fixme implement PointerOf types
		"cv_Mat_Mat_const_MatR_const_RangeX",    // duplicate of cv_Mat_Mat_Mat_VectorOfRange, but with pointers
		"cv_Mat_copySize_const_MatR",            // internal function
		"cv_Mat_propSize_MatSize",               // MatSize and MatStep types prevent assignment
		"cv_Mat_propStep_MatStep",               // MatSize and MatStep types prevent assignment
		"cv_Mat_push_back__const_voidX",         // internal method
		"cv_UMat_UMat_const_UMatR_const_RangeX", // duplicate of cv_UMat_UMat_UMat_VectorOfRange, but with pointers
		"cv_UMat_copySize_const_UMatR",          // internal function
		"cv_UMat_propSize_MatSize",              // MatSize and MatStep types prevent assignment
		"cv_UMat_propStep_MatStep",              // MatSize and MatStep types prevent assignment
		"cv_addImpl_int_const_charX",
		"cv_calcCovarMatrix_const_MatX_int_MatR_MatR_int_int", // duplicate of cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int, but with pointers
		"cv_cv_abs_short",
		"cv_detail_depthToString__int",                                   // detail function
		"cv_detail_typeToString__int",                                    // detail function
		"cv_fastFree_voidX",                                              // manual memory allocation
		"cv_fastMalloc_size_t",                                           // manual memory allocation
		"cv_format_const_charX",                                          // 3.2 accepts varargs, duplicate definition
		"cv_hconcat_const_MatX_size_t_const__OutputArrayR",               // duplicate of cv_hconcat_VectorOfMat_Mat, but with pointers
		"cv_merge_const_MatX_size_t_const__OutputArrayR", // duplicate of cv_merge_const__InputArrayR_const__OutputArrayR, but with pointers
		"cv_mixChannels_const_MatX_size_t_MatX_size_t_const_intX_size_t", // duplicate of cv_mixChannels_VectorOfMat_VectorOfMat_VectorOfint, but with pointers
		"cv_ocl_ProgramSource_ProgramSource_const_charX",                 // has duplicate with String
		"cv_setImpl_int",
		"cv_setUseCollection_bool",
		"cv_useCollection",
		"cv_vconcat_const_MatX_size_t_const__OutputArrayR", // duplicate of cv_vconcat_VectorOfMat_Mat, but with pointers
		// ### cudaimgproc ###
		"cv_cuda_histEven_const__InputArrayR_GpuMatXX_intXX_intXX_intXX_StreamR", // slice of boxed objects
		"cv_cuda_histRange_const__InputArrayR_GpuMatXX_const_GpuMatXX_StreamR",   // slice of boxed objects
		// ### dnn ###
		"cv_dnn_DictValue_DictValue_const_StringR", // effectively duplicate of cv_dnn_DictValue_DictValue_const_charX
		"cv_dnn_Layer_finalize_const_vectorLMatXGR_vectorLMatGR", // dup of cv_dnn_Layer_finalize_const__InputArrayR_const__OutputArrayR
		"cv_dnn_Model_operator_cv_dnn_Net_const",   // fixme, should generate fine, it's a dup of get_network_() anyway
		// ### face ###
		"cv_face_FacemarkLBF_Params_propPupils", // fixme array of vectors
		// ### gapi ###
		"cv_MediaFrame_IAdapter_access_Access", // use of deleted function ‘cv::MediaFrame::View::View(const cv::MediaFrame::View&)’
		"cv_MediaFrame_access_const_Access",    // use of deleted function ‘cv::MediaFrame::View::View(const cv::MediaFrame::View&)’
		"cv_RMat_Adapter_access_Access",        // use of deleted function ‘cv::RMat::View::View(const cv::RMat::View&)’
		"cv_RMat_IAdapter_access_Access",       // use of deleted function ‘cv::RMat::View::View(const cv::RMat::View&)’
		"cv_RMat_access_const_Access",          // use of deleted function ‘cv::RMat::View::View(const cv::RMat::View&)’
		// ### imgproc ###
		"cv_calcBackProject_const_MatX_int_const_intX_const_SparseMatR_const__OutputArrayR_const_floatXX_double_bool", // slice pointers
		"cv_calcBackProject_const_MatX_int_const_intX_const__InputArrayR_const__OutputArrayR_const_floatXX_double_bool", // slice pointers
		"cv_calcHist_const_MatX_int_const_intX_const__InputArrayR_SparseMatR_int_const_intX_const_floatXX_bool_bool",  // slice pointers
		"cv_calcHist_const_MatX_int_const_intX_const__InputArrayR_const__OutputArrayR_int_const_intX_const_floatXX_bool_bool", // slice pointers
		"cv_fillConvexPoly_MatR_const_PointX_int_const_ScalarR_int_int",                                                       // 3.2 3.4
		"cv_fillConvexPoly_const__InputOutputArrayR_const_PointX_int_const_ScalarR_int_int",
		"cv_fillPoly_MatR_const_PointXX_const_intX_int_const_ScalarR_int_int_Point", // 3.2
		"cv_fillPoly_const__InputOutputArrayR_const_PointXX_const_intX_int_const_ScalarR_int_int_Point",
		"cv_polylines_MatR_const_PointXX_const_intX_int_bool_const_ScalarR_int_int_int", // 3.2 3.4
		"cv_polylines_const__InputOutputArrayR_const_PointXX_const_intX_int_bool_const_ScalarR_int_int_int",
		// ### objdetect ###
		"cv_QRCodeDetector_detectAndDecodeMulti_const_const__InputArrayR_vectorLstringGR_const__OutputArrayR_const__OutputArrayR", // fixme: stores data to Vector<String>, that doesn't work yet
		// ### optflow ###
		"cv_optflow_GPCTrainingSamples_operator_cv_optflow_GPCSamplesVector", // support of "operator &" missing
		// ### text ###
		"cv_text_ERStat_propPixels", // fixme: reference to a vector, we don't handle it too well yet
		"cv_text_ERStat_propPixels_vectorLintGX", // fixme: reference to a vector, we don't handle it too well yet
		// those function are marked as CV_EXPORTS, but they are missing from the shared libraries
		// ### core ###
		"cv_MatConstIterator_MatConstIterator_const_MatX_const_intX",
		"cv_SparseMatConstIterator_operatorSS",
		"cv_SparseMatIterator_SparseMatIterator_SparseMatX_const_intX",
		"cv__OutputArray__OutputArray_const_vectorLGpuMatGR",
		"cv_cuda_convertFp16_const__InputArrayR_const__OutputArrayR_StreamR",
		"cv_getImpl_vectorLintGR_vectorLStringGR",
		// ### dnn ###
		"cv_dnn_BackendNode_BackendNode_int",
		// ### stitching ###
		"cv_createStitcherScans_bool",
		"cv_createStitcher_bool",
		// ### surface_matching ###
		"cv_ppf_match_3d_PPF3DDetector_read_const_FileNodeR",
		"cv_ppf_match_3d_PPF3DDetector_write_const_FileStorageR",
		// ### tracking ###
		"cv_CvFeatureParams_CvFeatureParams",
		"cv_CvFeatureParams_create_FeatureType",
		"cv_CvFeatureParams_create_int",
		"cv_CvHaarEvaluator_FeatureHaar_FeatureHaar_Size",
		"cv_CvHaarEvaluator_FeatureHaar_eval_const_const_MatR_Rect_floatX",
		"cv_CvHaarEvaluator_FeatureHaar_getAreas_const",
		"cv_CvHaarEvaluator_FeatureHaar_getInitMean_const",
		"cv_CvHaarEvaluator_FeatureHaar_getInitSigma_const",
		"cv_CvHaarEvaluator_FeatureHaar_getNumAreas",
		"cv_CvHaarEvaluator_FeatureHaar_getWeights_const",
		"cv_CvHaarEvaluator_getFeatures_const",
		"cv_CvHaarEvaluator_setWinSize_Size",
		"cv_CvHaarEvaluator_setWinSize_const",
		"cv_CvHaarEvaluator_writeFeature_const_FileStorageR",
		"cv_SparseMatConstIterator_operatorSS",
		"cv__OutputArray__OutputArray_const_vectorLGpuMatGR",
	])
});
