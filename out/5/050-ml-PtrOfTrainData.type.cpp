extern "C" {
	// cv::Ptr<cv::ml::TrainData>::getInnerPtr() generated
	// ("cv::Ptr<cv::ml::TrainData>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ml::TrainData* cv_PtrLcv_ml_TrainDataG_getInnerPtr_const(const cv::Ptr<cv::ml::TrainData>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::TrainData>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ml::TrainData>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ml::TrainData* cv_PtrLcv_ml_TrainDataG_getInnerPtrMut(cv::Ptr<cv::ml::TrainData>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ml::TrainData>::new_null() generated
	// ("cv::Ptr<cv::ml::TrainData>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ml::TrainData>* cv_PtrLcv_ml_TrainDataG_new_null_const() {
			return new cv::Ptr<cv::ml::TrainData>();
	}

	// cv::Ptr<cv::ml::TrainData>::delete() generated
	// ("cv::Ptr<cv::ml::TrainData>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ml_TrainDataG_delete(cv::Ptr<cv::ml::TrainData>* instance) {
			delete instance;
	}

}

