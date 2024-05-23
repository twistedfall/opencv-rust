extern "C" {
	// cv::Ptr<cv::ml::NormalBayesClassifier>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::NormalBayesClassifier* cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtr_const(const cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::NormalBayesClassifier>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::NormalBayesClassifier* cv_PtrLcv_ml_NormalBayesClassifierG_getInnerPtrMut(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::NormalBayesClassifier>::new_null() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::NormalBayesClassifier>* cv_PtrLcv_ml_NormalBayesClassifierG_new_null_const() {
			return new cv::Ptr<cv::ml::NormalBayesClassifier>();
	}

	// cv::Ptr<cv::ml::NormalBayesClassifier>::delete() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_NormalBayesClassifierG_delete(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::NormalBayesClassifier>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::NormalBayesClassifier>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::NormalBayesClassifier>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_NormalBayesClassifierG_to_PtrOfStatModel(cv::Ptr<cv::ml::NormalBayesClassifier>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

