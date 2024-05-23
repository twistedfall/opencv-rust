extern "C" {
	// cv::Ptr<cv::ml::EM>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::EM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::EM* cv_PtrLcv_ml_EMG_getInnerPtr_const(const cv::Ptr<cv::ml::EM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::EM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::EM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::EM* cv_PtrLcv_ml_EMG_getInnerPtrMut(cv::Ptr<cv::ml::EM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::EM>::new_null() generated
	// ("cv::Ptr<cv::ml::EM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::EM>* cv_PtrLcv_ml_EMG_new_null_const() {
			return new cv::Ptr<cv::ml::EM>();
	}

	// cv::Ptr<cv::ml::EM>::delete() generated
	// ("cv::Ptr<cv::ml::EM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_EMG_delete(cv::Ptr<cv::ml::EM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::EM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::EM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_EMG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::EM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::EM>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::EM>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_EMG_to_PtrOfStatModel(cv::Ptr<cv::ml::EM>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

