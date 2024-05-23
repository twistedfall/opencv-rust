extern "C" {
	// cv::Ptr<cv::ml::DTrees>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::DTrees>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::DTrees* cv_PtrLcv_ml_DTreesG_getInnerPtr_const(const cv::Ptr<cv::ml::DTrees>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::DTrees>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::DTrees>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::DTrees* cv_PtrLcv_ml_DTreesG_getInnerPtrMut(cv::Ptr<cv::ml::DTrees>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::DTrees>::new_null() generated
	// ("cv::Ptr<cv::ml::DTrees>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::DTrees>* cv_PtrLcv_ml_DTreesG_new_null_const() {
			return new cv::Ptr<cv::ml::DTrees>();
	}

	// cv::Ptr<cv::ml::DTrees>::delete() generated
	// ("cv::Ptr<cv::ml::DTrees>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_DTreesG_delete(cv::Ptr<cv::ml::DTrees>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::DTrees>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::DTrees>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_DTreesG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::DTrees>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::DTrees>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::DTrees>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_DTreesG_to_PtrOfStatModel(cv::Ptr<cv::ml::DTrees>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

