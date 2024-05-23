extern "C" {
	// cv::Ptr<cv::dnn::Graph>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::Graph>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::Graph* cv_PtrLcv_dnn_GraphG_getInnerPtr_const(const cv::Ptr<cv::dnn::Graph>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Graph>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::Graph>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::Graph* cv_PtrLcv_dnn_GraphG_getInnerPtrMut(cv::Ptr<cv::dnn::Graph>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::Graph>::new_null() generated
	// ("cv::Ptr<cv::dnn::Graph>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::Graph>* cv_PtrLcv_dnn_GraphG_new_null_const() {
			return new cv::Ptr<cv::dnn::Graph>();
	}

	// cv::Ptr<cv::dnn::Graph>::delete() generated
	// ("cv::Ptr<cv::dnn::Graph>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_GraphG_delete(cv::Ptr<cv::dnn::Graph>* instance) {
			delete instance;
	}

}

