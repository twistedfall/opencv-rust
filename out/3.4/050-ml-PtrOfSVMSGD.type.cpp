extern "C" {
	// cv::Ptr<cv::ml::SVMSGD>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::SVMSGD* cv_PtrLcv_ml_SVMSGDG_getInnerPtr_const(const cv::Ptr<cv::ml::SVMSGD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVMSGD>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::SVMSGD* cv_PtrLcv_ml_SVMSGDG_getInnerPtrMut(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVMSGD>::new_null() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::SVMSGD>* cv_PtrLcv_ml_SVMSGDG_new_null_const() {
			return new cv::Ptr<cv::ml::SVMSGD>();
	}

	// cv::Ptr<cv::ml::SVMSGD>::delete() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_SVMSGDG_delete(cv::Ptr<cv::ml::SVMSGD>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::SVMSGD>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVMSGDG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::SVMSGD>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::SVMSGD>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_SVMSGDG_to_PtrOfStatModel(cv::Ptr<cv::ml::SVMSGD>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

