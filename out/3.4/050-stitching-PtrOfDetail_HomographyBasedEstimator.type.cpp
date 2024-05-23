extern "C" {
	// cv::Ptr<cv::detail::HomographyBasedEstimator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::HomographyBasedEstimator* cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtr_const(const cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::HomographyBasedEstimator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::HomographyBasedEstimator* cv_PtrLcv_detail_HomographyBasedEstimatorG_getInnerPtrMut(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::HomographyBasedEstimator>::new_null() generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::HomographyBasedEstimator>* cv_PtrLcv_detail_HomographyBasedEstimatorG_new_null_const() {
			return new cv::Ptr<cv::detail::HomographyBasedEstimator>();
	}

	// cv::Ptr<cv::detail::HomographyBasedEstimator>::delete() generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_HomographyBasedEstimatorG_delete(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::HomographyBasedEstimator>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_HomographyBasedEstimatorG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::HomographyBasedEstimator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::HomographyBasedEstimator>::new", vec![(pred!(const, ["val"], ["cv::detail::HomographyBasedEstimator"]), _)]),
	cv::Ptr<cv::detail::HomographyBasedEstimator>* cv_PtrLcv_detail_HomographyBasedEstimatorG_new_const_HomographyBasedEstimator(cv::detail::HomographyBasedEstimator* val) {
			return new cv::Ptr<cv::detail::HomographyBasedEstimator>(val);
	}

}

