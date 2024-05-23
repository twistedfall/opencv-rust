extern "C" {
	// cv::Ptr<cv::dnn::BackendWrapper>::getInnerPtr() generated
	// ("cv::Ptr<cv::dnn::BackendWrapper>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dnn::BackendWrapper* cv_PtrLcv_dnn_BackendWrapperG_getInnerPtr_const(const cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BackendWrapper>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dnn::BackendWrapper>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dnn::BackendWrapper* cv_PtrLcv_dnn_BackendWrapperG_getInnerPtrMut(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dnn::BackendWrapper>::new_null() generated
	// ("cv::Ptr<cv::dnn::BackendWrapper>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dnn::BackendWrapper>* cv_PtrLcv_dnn_BackendWrapperG_new_null_const() {
			return new cv::Ptr<cv::dnn::BackendWrapper>();
	}

	// cv::Ptr<cv::dnn::BackendWrapper>::delete() generated
	// ("cv::Ptr<cv::dnn::BackendWrapper>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dnn_BackendWrapperG_delete(cv::Ptr<cv::dnn::BackendWrapper>* instance) {
			delete instance;
	}

}

