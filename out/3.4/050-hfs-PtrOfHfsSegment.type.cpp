extern "C" {
	// cv::Ptr<cv::hfs::HfsSegment>::getInnerPtr() generated
	// ("cv::Ptr<cv::hfs::HfsSegment>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::hfs::HfsSegment* cv_PtrLcv_hfs_HfsSegmentG_getInnerPtr_const(const cv::Ptr<cv::hfs::HfsSegment>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::hfs::HfsSegment>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::hfs::HfsSegment>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::hfs::HfsSegment* cv_PtrLcv_hfs_HfsSegmentG_getInnerPtrMut(cv::Ptr<cv::hfs::HfsSegment>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::hfs::HfsSegment>::new_null() generated
	// ("cv::Ptr<cv::hfs::HfsSegment>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::hfs::HfsSegment>* cv_PtrLcv_hfs_HfsSegmentG_new_null_const() {
			return new cv::Ptr<cv::hfs::HfsSegment>();
	}

	// cv::Ptr<cv::hfs::HfsSegment>::delete() generated
	// ("cv::Ptr<cv::hfs::HfsSegment>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_hfs_HfsSegmentG_delete(cv::Ptr<cv::hfs::HfsSegment>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::hfs::HfsSegment>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::hfs::HfsSegment>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_hfs_HfsSegmentG_to_PtrOfAlgorithm(cv::Ptr<cv::hfs::HfsSegment>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

