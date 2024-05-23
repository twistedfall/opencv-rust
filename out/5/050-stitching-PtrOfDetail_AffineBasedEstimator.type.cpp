extern "C" {
	// cv::Ptr<cv::detail::AffineBasedEstimator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::AffineBasedEstimator* cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtr_const(const cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AffineBasedEstimator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineBasedEstimator* cv_PtrLcv_detail_AffineBasedEstimatorG_getInnerPtrMut(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AffineBasedEstimator>::new_null() generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::AffineBasedEstimator>* cv_PtrLcv_detail_AffineBasedEstimatorG_new_null_const() {
			return new cv::Ptr<cv::detail::AffineBasedEstimator>();
	}

	// cv::Ptr<cv::detail::AffineBasedEstimator>::delete() generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_AffineBasedEstimatorG_delete(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::AffineBasedEstimator>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_AffineBasedEstimatorG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::AffineBasedEstimator>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::AffineBasedEstimator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::AffineBasedEstimator>::new", vec![(pred!(const, ["val"], ["cv::detail::AffineBasedEstimator"]), _)]),
	cv::Ptr<cv::detail::AffineBasedEstimator>* cv_PtrLcv_detail_AffineBasedEstimatorG_new_const_AffineBasedEstimator(cv::detail::AffineBasedEstimator* val) {
			return new cv::Ptr<cv::detail::AffineBasedEstimator>(val);
	}

}

