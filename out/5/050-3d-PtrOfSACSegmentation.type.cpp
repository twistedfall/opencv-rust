extern "C" {
	// cv::Ptr<cv::SACSegmentation>::getInnerPtr() generated
	// ("cv::Ptr<cv::SACSegmentation>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::SACSegmentation* cv_PtrLcv_SACSegmentationG_getInnerPtr_const(const cv::Ptr<cv::SACSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SACSegmentation>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::SACSegmentation>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::SACSegmentation* cv_PtrLcv_SACSegmentationG_getInnerPtrMut(cv::Ptr<cv::SACSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::SACSegmentation>::new_null() generated
	// ("cv::Ptr<cv::SACSegmentation>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::SACSegmentation>* cv_PtrLcv_SACSegmentationG_new_null_const() {
			return new cv::Ptr<cv::SACSegmentation>();
	}

	// cv::Ptr<cv::SACSegmentation>::delete() generated
	// ("cv::Ptr<cv::SACSegmentation>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_SACSegmentationG_delete(cv::Ptr<cv::SACSegmentation>* instance) {
			delete instance;
	}

}

