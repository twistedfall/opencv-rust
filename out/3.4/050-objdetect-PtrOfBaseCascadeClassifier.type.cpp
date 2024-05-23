extern "C" {
	// cv::Ptr<cv::BaseCascadeClassifier>::getInnerPtr() generated
	// ("cv::Ptr<cv::BaseCascadeClassifier>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::BaseCascadeClassifier* cv_PtrLcv_BaseCascadeClassifierG_getInnerPtr_const(const cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BaseCascadeClassifier>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::BaseCascadeClassifier>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::BaseCascadeClassifier* cv_PtrLcv_BaseCascadeClassifierG_getInnerPtrMut(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::BaseCascadeClassifier>::new_null() generated
	// ("cv::Ptr<cv::BaseCascadeClassifier>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::BaseCascadeClassifier>* cv_PtrLcv_BaseCascadeClassifierG_new_null_const() {
			return new cv::Ptr<cv::BaseCascadeClassifier>();
	}

	// cv::Ptr<cv::BaseCascadeClassifier>::delete() generated
	// ("cv::Ptr<cv::BaseCascadeClassifier>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_BaseCascadeClassifierG_delete(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::BaseCascadeClassifier>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::BaseCascadeClassifier>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_BaseCascadeClassifierG_to_PtrOfAlgorithm(cv::Ptr<cv::BaseCascadeClassifier>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

