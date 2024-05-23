extern "C" {
	// cv::Ptr<cv::ml::ANN_MLP>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::ANN_MLP* cv_PtrLcv_ml_ANN_MLPG_getInnerPtr_const(const cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ANN_MLP>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP* cv_PtrLcv_ml_ANN_MLPG_getInnerPtrMut(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ANN_MLP>::new_null() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::ANN_MLP>* cv_PtrLcv_ml_ANN_MLPG_new_null_const() {
			return new cv::Ptr<cv::ml::ANN_MLP>();
	}

	// cv::Ptr<cv::ml::ANN_MLP>::delete() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_ANN_MLPG_delete(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::ANN_MLP>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_ANN_MLPG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::ANN_MLP>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::ANN_MLP>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_ANN_MLPG_to_PtrOfStatModel(cv::Ptr<cv::ml::ANN_MLP>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

