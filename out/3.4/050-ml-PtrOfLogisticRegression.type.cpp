extern "C" {
	// cv::Ptr<cv::ml::LogisticRegression>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::LogisticRegression* cv_PtrLcv_ml_LogisticRegressionG_getInnerPtr_const(const cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::LogisticRegression>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::LogisticRegression* cv_PtrLcv_ml_LogisticRegressionG_getInnerPtrMut(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::LogisticRegression>::new_null() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::LogisticRegression>* cv_PtrLcv_ml_LogisticRegressionG_new_null_const() {
			return new cv::Ptr<cv::ml::LogisticRegression>();
	}

	// cv::Ptr<cv::ml::LogisticRegression>::delete() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_LogisticRegressionG_delete(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::LogisticRegression>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::LogisticRegression>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::LogisticRegression>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_LogisticRegressionG_to_PtrOfStatModel(cv::Ptr<cv::ml::LogisticRegression>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

