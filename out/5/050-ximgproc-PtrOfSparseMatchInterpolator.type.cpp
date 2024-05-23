extern "C" {
	// cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::SparseMatchInterpolator* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SparseMatchInterpolator* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::SparseMatchInterpolator>();
	}

	// cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::delete() generated
	// ("cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_delete(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::SparseMatchInterpolator>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SparseMatchInterpolatorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SparseMatchInterpolator>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

