extern "C" {
	// cv::Ptr<cv::SIFT>::getInnerPtr() generated
	// ("cv::Ptr<cv::SIFT>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SIFT* cv_PtrLcv_SIFTG_getInnerPtr_const(const cv::Ptr<cv::SIFT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SIFT>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SIFT>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SIFT* cv_PtrLcv_SIFTG_getInnerPtrMut(cv::Ptr<cv::SIFT>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SIFT>::new_null() generated
	// ("cv::Ptr<cv::SIFT>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SIFT>* cv_PtrLcv_SIFTG_new_null_const() {
			return new cv::Ptr<cv::SIFT>();
	}

	// cv::Ptr<cv::SIFT>::delete() generated
	// ("cv::Ptr<cv::SIFT>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SIFTG_delete(cv::Ptr<cv::SIFT>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::SIFT>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::SIFT>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_SIFTG_to_PtrOfAlgorithm(cv::Ptr<cv::SIFT>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::SIFT>::to_PtrOfFeature2D() generated
	// ("cv::Ptr<cv::SIFT>::to_PtrOfFeature2D", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Feature2D>* cv_PtrLcv_SIFTG_to_PtrOfFeature2D(cv::Ptr<cv::SIFT>* instance) {
			return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}

}

