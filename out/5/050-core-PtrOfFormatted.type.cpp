extern "C" {
	// cv::Ptr<cv::Formatted>::getInnerPtr() generated
	// ("cv::Ptr<cv::Formatted>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::Formatted* cv_PtrLcv_FormattedG_getInnerPtr_const(const cv::Ptr<cv::Formatted>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Formatted>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::Formatted>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::Formatted* cv_PtrLcv_FormattedG_getInnerPtrMut(cv::Ptr<cv::Formatted>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::Formatted>::new_null() generated
	// ("cv::Ptr<cv::Formatted>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::Formatted>* cv_PtrLcv_FormattedG_new_null_const() {
			return new cv::Ptr<cv::Formatted>();
	}

	// cv::Ptr<cv::Formatted>::delete() generated
	// ("cv::Ptr<cv::Formatted>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FormattedG_delete(cv::Ptr<cv::Formatted>* instance) {
			delete instance;
	}

}

