extern "C" {
	// cv::Ptr<cv::ml::KNearest>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::KNearest>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::KNearest* cv_PtrLcv_ml_KNearestG_getInnerPtr_const(const cv::Ptr<cv::ml::KNearest>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::KNearest>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::KNearest>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::KNearest* cv_PtrLcv_ml_KNearestG_getInnerPtrMut(cv::Ptr<cv::ml::KNearest>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::KNearest>::new_null() generated
	// ("cv::Ptr<cv::ml::KNearest>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::KNearest>* cv_PtrLcv_ml_KNearestG_new_null_const() {
			return new cv::Ptr<cv::ml::KNearest>();
	}

	// cv::Ptr<cv::ml::KNearest>::delete() generated
	// ("cv::Ptr<cv::ml::KNearest>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_KNearestG_delete(cv::Ptr<cv::ml::KNearest>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ml::KNearest>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ml::KNearest>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ml_KNearestG_to_PtrOfAlgorithm(cv::Ptr<cv::ml::KNearest>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ml::KNearest>::to_PtrOfStatModel() generated
	// ("cv::Ptr<cv::ml::KNearest>::to_PtrOfStatModel", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ml::StatModel>* cv_PtrLcv_ml_KNearestG_to_PtrOfStatModel(cv::Ptr<cv::ml::KNearest>* instance) {
			return new cv::Ptr<cv::ml::StatModel>(instance->dynamicCast<cv::ml::StatModel>());
	}

}

