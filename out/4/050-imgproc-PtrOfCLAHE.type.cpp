extern "C" {
	// cv::Ptr<cv::CLAHE>::getInnerPtr() generated
	// ("cv::Ptr<cv::CLAHE>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CLAHE* cv_PtrLcv_CLAHEG_getInnerPtr_const(const cv::Ptr<cv::CLAHE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CLAHE>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CLAHE>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CLAHE* cv_PtrLcv_CLAHEG_getInnerPtrMut(cv::Ptr<cv::CLAHE>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CLAHE>::new_null() generated
	// ("cv::Ptr<cv::CLAHE>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CLAHE>* cv_PtrLcv_CLAHEG_new_null_const() {
			return new cv::Ptr<cv::CLAHE>();
	}

	// cv::Ptr<cv::CLAHE>::delete() generated
	// ("cv::Ptr<cv::CLAHE>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CLAHEG_delete(cv::Ptr<cv::CLAHE>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CLAHE>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::CLAHE>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CLAHEG_to_PtrOfAlgorithm(cv::Ptr<cv::CLAHE>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

