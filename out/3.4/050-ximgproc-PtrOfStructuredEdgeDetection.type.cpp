extern "C" {
	// cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::StructuredEdgeDetection* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::StructuredEdgeDetection* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_getInnerPtrMut(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::StructuredEdgeDetection>();
	}

	// cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::delete() generated
	// ("cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_delete(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::StructuredEdgeDetection>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_StructuredEdgeDetectionG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::StructuredEdgeDetection>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

