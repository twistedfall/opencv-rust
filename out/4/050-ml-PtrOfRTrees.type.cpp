extern "C" {
	// cv::Ptr<cv::ml::RTrees>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::RTrees>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::RTrees* cv_PtrLcv_ml_RTreesG_getInnerPtr_const(const cv::Ptr<cv::ml::RTrees>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::RTrees>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::RTrees>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::RTrees* cv_PtrLcv_ml_RTreesG_getInnerPtrMut(cv::Ptr<cv::ml::RTrees>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::RTrees>::new_null() generated
	// ("cv::Ptr<cv::ml::RTrees>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::RTrees>* cv_PtrLcv_ml_RTreesG_new_null_const() {
			return new cv::Ptr<cv::ml::RTrees>();
	}

	// cv::Ptr<cv::ml::RTrees>::delete() generated
	// ("cv::Ptr<cv::ml::RTrees>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_RTreesG_delete(cv::Ptr<cv::ml::RTrees>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::RTrees>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::RTrees>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_RTreesG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::RTrees>::to_PtrOfDTrees() generated
	// ("cv::Ptr<cv::ml::RTrees>::to_PtrOfDTrees", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::DTrees>* cv_PtrLcv_ml_RTreesG_to_PtrOfDTrees(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::ml::DTrees>(instance->dynamicCast<cv::ml::DTrees>());
	}

	// cv::Ptr<cv::ml::RTrees>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::RTrees>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_RTreesG_to_PtrOfStatModel(cv::Ptr<cv::ml::RTrees>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

