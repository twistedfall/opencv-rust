extern "C" {
	// cv::Ptr<cv::LineSegmentDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::LineSegmentDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::LineSegmentDetector* cv_PtrLcv_LineSegmentDetectorG_getInnerPtr_const(const cv::Ptr<cv::LineSegmentDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::LineSegmentDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::LineSegmentDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::LineSegmentDetector* cv_PtrLcv_LineSegmentDetectorG_getInnerPtrMut(cv::Ptr<cv::LineSegmentDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::LineSegmentDetector>::new_null() generated
	// ("cv::Ptr<cv::LineSegmentDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::LineSegmentDetector>* cv_PtrLcv_LineSegmentDetectorG_new_null_const() {
			return new cv::Ptr<cv::LineSegmentDetector>();
	}

	// cv::Ptr<cv::LineSegmentDetector>::delete() generated
	// ("cv::Ptr<cv::LineSegmentDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_LineSegmentDetectorG_delete(cv::Ptr<cv::LineSegmentDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::LineSegmentDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::LineSegmentDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_LineSegmentDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::LineSegmentDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

