extern "C" {
	// cv::Ptr<cv::ml::SVM>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::SVM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::SVM* cv_PtrLcv_ml_SVMG_getInnerPtr_const(const cv::Ptr<cv::ml::SVM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::SVM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::SVM* cv_PtrLcv_ml_SVMG_getInnerPtrMut(cv::Ptr<cv::ml::SVM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::SVM>::new_null() generated
	// ("cv::Ptr<cv::ml::SVM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::SVM>* cv_PtrLcv_ml_SVMG_new_null_const() {
			return new cv::Ptr<cv::ml::SVM>();
	}

	// cv::Ptr<cv::ml::SVM>::delete() generated
	// ("cv::Ptr<cv::ml::SVM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_SVMG_delete(cv::Ptr<cv::ml::SVM>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::SVM>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::SVM>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_SVMG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::SVM>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::SVM>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::SVM>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_SVMG_to_PtrOfStatModel(cv::Ptr<cv::ml::SVM>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

