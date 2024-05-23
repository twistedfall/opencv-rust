extern "C" {
	// cv::Ptr<cv::ximgproc::RICInterpolator>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::RICInterpolator* cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RICInterpolator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::RICInterpolator* cv_PtrLcv_ximgproc_RICInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RICInterpolator>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::RICInterpolator>* cv_PtrLcv_ximgproc_RICInterpolatorG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::RICInterpolator>();
	}

	// cv::Ptr<cv::ximgproc::RICInterpolator>::delete() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_RICInterpolatorG_delete(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::RICInterpolator>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::RICInterpolator>::to_PtrOfSparseMatchInterpolator() generated
	// ("cv::Ptr<cv::ximgproc::RICInterpolator>::to_PtrOfSparseMatchInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* cv_PtrLcv_ximgproc_RICInterpolatorG_to_PtrOfSparseMatchInterpolator(cv::Ptr<cv::ximgproc::RICInterpolator>* instance) {
			return new cv::Ptr<cv::ximgproc::SparseMatchInterpolator>(instance->dynamicCast<cv::ximgproc::SparseMatchInterpolator>());
	}

}

