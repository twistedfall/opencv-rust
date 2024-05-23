extern "C" {
	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::ANN_MLP_ANNEAL* cv_PtrLcv_ml_ANN_MLP_ANNEALG_getInnerPtr_const(const cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::ANN_MLP_ANNEAL* cv_PtrLcv_ml_ANN_MLP_ANNEALG_getInnerPtrMut(cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::new_null() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* cv_PtrLcv_ml_ANN_MLP_ANNEALG_new_null_const() {
			return new cv::Ptr<cv::ml::ANN_MLP_ANNEAL>();
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::delete() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_ANN_MLP_ANNEALG_delete(cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfANN_MLP() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfANN_MLP", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::ANN_MLP>* cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfANN_MLP(cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			return new cv::Ptr<cv::ml::ANN_MLP>(instance->dynamicCast<cv::ml::ANN_MLP>());
	}

	// cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::ANN_MLP_ANNEAL>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_ANN_MLP_ANNEALG_to_PtrOfStatModel(cv::Ptr<cv::ml::ANN_MLP_ANNEAL>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

