extern "C" {
	// cv::Ptr<cv::ml::Boost>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::Boost>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::Boost* cv_PtrLcv_ml_BoostG_getInnerPtr_const(const cv::Ptr<cv::ml::Boost>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::Boost>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::Boost>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::Boost* cv_PtrLcv_ml_BoostG_getInnerPtrMut(cv::Ptr<cv::ml::Boost>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::Boost>::new_null() generated
	// ("cv::Ptr<cv::ml::Boost>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::Boost>* cv_PtrLcv_ml_BoostG_new_null_const() {
			return new cv::Ptr<cv::ml::Boost>();
	}

	// cv::Ptr<cv::ml::Boost>::delete() generated
	// ("cv::Ptr<cv::ml::Boost>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_BoostG_delete(cv::Ptr<cv::ml::Boost>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::Boost>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::Boost>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_BoostG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::Boost>::to_PtrOfDTrees() generated
	// ("cv::Ptr<cv::ml::Boost>::to_PtrOfDTrees", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::DTrees>* cv_PtrLcv_ml_BoostG_to_PtrOfDTrees(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::ml::DTrees>(instance->dynamicCast<cv::ml::DTrees>());
	}

	// cv::Ptr<cv::ml::Boost>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::Boost>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_BoostG_to_PtrOfStatModel(cv::Ptr<cv::ml::Boost>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

