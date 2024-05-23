extern "C" {
	// cv::Ptr<cv::Formatter>::getInnerPtr() generated
	// ("cv::Ptr<cv::Formatter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Formatter* cv_PtrLcv_FormatterG_getInnerPtr_const(const cv::Ptr<cv::Formatter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Formatter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Formatter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Formatter* cv_PtrLcv_FormatterG_getInnerPtrMut(cv::Ptr<cv::Formatter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Formatter>::new_null() generated
	// ("cv::Ptr<cv::Formatter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Formatter>* cv_PtrLcv_FormatterG_new_null_const() {
			return new cv::Ptr<cv::Formatter>();
	}

	// cv::Ptr<cv::Formatter>::delete() generated
	// ("cv::Ptr<cv::Formatter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FormatterG_delete(cv::Ptr<cv::Formatter>* instance) {
			delete instance;
	}

}

