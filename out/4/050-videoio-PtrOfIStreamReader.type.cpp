extern "C" {
	// cv::Ptr<cv::IStreamReader>::getInnerPtr() generated
	// ("cv::Ptr<cv::IStreamReader>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::IStreamReader* cv_PtrLcv_IStreamReaderG_getInnerPtr_const(const cv::Ptr<cv::IStreamReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::IStreamReader>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::IStreamReader>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::IStreamReader* cv_PtrLcv_IStreamReaderG_getInnerPtrMut(cv::Ptr<cv::IStreamReader>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::IStreamReader>::new_null() generated
	// ("cv::Ptr<cv::IStreamReader>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::IStreamReader>* cv_PtrLcv_IStreamReaderG_new_null_const() {
			return new cv::Ptr<cv::IStreamReader>();
	}

	// cv::Ptr<cv::IStreamReader>::delete() generated
	// ("cv::Ptr<cv::IStreamReader>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_IStreamReaderG_delete(cv::Ptr<cv::IStreamReader>* instance) {
			delete instance;
	}

}

