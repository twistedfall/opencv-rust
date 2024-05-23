extern "C" {
	// cv::Ptr<cv::SimpleBlobDetector>::getInnerPtr() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SimpleBlobDetector* cv_PtrLcv_SimpleBlobDetectorG_getInnerPtr_const(const cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SimpleBlobDetector>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SimpleBlobDetector* cv_PtrLcv_SimpleBlobDetectorG_getInnerPtrMut(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SimpleBlobDetector>::new_null() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SimpleBlobDetector>* cv_PtrLcv_SimpleBlobDetectorG_new_null_const() {
			return new cv::Ptr<cv::SimpleBlobDetector>();
	}

	// cv::Ptr<cv::SimpleBlobDetector>::delete() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SimpleBlobDetectorG_delete(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SimpleBlobDetector>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfAlgorithm(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::SimpleBlobDetector>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::SimpleBlobDetector>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_SimpleBlobDetectorG_to_PtrOfFeature2D(cv::Ptr<cv::SimpleBlobDetector>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

