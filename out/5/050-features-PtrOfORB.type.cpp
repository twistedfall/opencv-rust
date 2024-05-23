extern "C" {
	// cv::Ptr<cv::ORB>::getInnerPtr() generated
	// ("cv::Ptr<cv::ORB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ORB* cv_PtrLcv_ORBG_getInnerPtr_const(const cv::Ptr<cv::ORB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ORB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ORB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ORB* cv_PtrLcv_ORBG_getInnerPtrMut(cv::Ptr<cv::ORB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ORB>::new_null() generated
	// ("cv::Ptr<cv::ORB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ORB>* cv_PtrLcv_ORBG_new_null_const() {
			return new cv::Ptr<cv::ORB>();
	}

	// cv::Ptr<cv::ORB>::delete() generated
	// ("cv::Ptr<cv::ORB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ORBG_delete(cv::Ptr<cv::ORB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ORB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ORB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ORBG_to_PtrOfAlgorithm(cv::Ptr<cv::ORB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ORB>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::ORB>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_ORBG_to_PtrOfFeature2D(cv::Ptr<cv::ORB>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

