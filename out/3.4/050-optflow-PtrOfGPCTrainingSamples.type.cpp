extern "C" {
	// cv::Ptr<cv::optflow::GPCTrainingSamples>::getInnerPtr() generated
	// ("cv::Ptr<cv::optflow::GPCTrainingSamples>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::optflow::GPCTrainingSamples* cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtr_const(const cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::GPCTrainingSamples>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::optflow::GPCTrainingSamples>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::optflow::GPCTrainingSamples* cv_PtrLcv_optflow_GPCTrainingSamplesG_getInnerPtrMut(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::optflow::GPCTrainingSamples>::new_null() generated
	// ("cv::Ptr<cv::optflow::GPCTrainingSamples>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::optflow::GPCTrainingSamples>* cv_PtrLcv_optflow_GPCTrainingSamplesG_new_null_const() {
			return new cv::Ptr<cv::optflow::GPCTrainingSamples>();
	}

	// cv::Ptr<cv::optflow::GPCTrainingSamples>::delete() generated
	// ("cv::Ptr<cv::optflow::GPCTrainingSamples>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_optflow_GPCTrainingSamplesG_delete(cv::Ptr<cv::optflow::GPCTrainingSamples>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::optflow::GPCTrainingSamples>::new(TraitClass) generated
	// ("cv::Ptr<cv::optflow::GPCTrainingSamples>::new", vec![(pred!(const, ["val"], ["cv::optflow::GPCTrainingSamples"]), _)]),
	cv::Ptr<cv::optflow::GPCTrainingSamples>* cv_PtrLcv_optflow_GPCTrainingSamplesG_new_const_GPCTrainingSamples(cv::optflow::GPCTrainingSamples* val) {
			return new cv::Ptr<cv::optflow::GPCTrainingSamples>(val);
	}

}

