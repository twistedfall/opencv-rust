extern "C" {
	// cv::Ptr<cv::ximgproc::ScanSegment>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::ScanSegment>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::ScanSegment* cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::ScanSegment>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::ScanSegment>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::ScanSegment* cv_PtrLcv_ximgproc_ScanSegmentG_getInnerPtrMut(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::ScanSegment>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::ScanSegment>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::ScanSegment>* cv_PtrLcv_ximgproc_ScanSegmentG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::ScanSegment>();
	}

	// cv::Ptr<cv::ximgproc::ScanSegment>::delete() generated
	// ("cv::Ptr<cv::ximgproc::ScanSegment>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_ScanSegmentG_delete(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::ScanSegment>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::ScanSegment>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_ScanSegmentG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

