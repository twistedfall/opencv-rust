extern "C" {
	// cv::Ptr<cv::ANNIndex>::getInnerPtr() generated
	// ("cv::Ptr<cv::ANNIndex>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ANNIndex* cv_PtrLcv_ANNIndexG_getInnerPtr_const(const cv::Ptr<cv::ANNIndex>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ANNIndex>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ANNIndex>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ANNIndex* cv_PtrLcv_ANNIndexG_getInnerPtrMut(cv::Ptr<cv::ANNIndex>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ANNIndex>::new_null() generated
	// ("cv::Ptr<cv::ANNIndex>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ANNIndex>* cv_PtrLcv_ANNIndexG_new_null_const() {
			return new cv::Ptr<cv::ANNIndex>();
	}

	// cv::Ptr<cv::ANNIndex>::delete() generated
	// ("cv::Ptr<cv::ANNIndex>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ANNIndexG_delete(cv::Ptr<cv::ANNIndex>* instance) {
			delete instance;
	}

}

