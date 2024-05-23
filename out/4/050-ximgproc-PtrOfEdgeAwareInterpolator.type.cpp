extern "C" {
	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::EdgeAwareInterpolator* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::EdgeAwareInterpolator* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>();
	}

	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::delete() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_delete(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::to_PtrOfSparseMatchInterpolator() generated
	// ("cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>::to_PtrOfSparseMatchInterpolator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* cv_PtrLcv_ximgproc_EdgeAwareInterpolatorG_to_PtrOfSparseMatchInterpolator(cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>* instance) {
			return new cv::Ptr<cv::ximgproc::SparseMatchInterpolator>(instance->dynamicCast<cv::ximgproc::SparseMatchInterpolator>());
	}

}

