extern "C" {
	// cv::Ptr<cv::WarperCreator>::getInnerPtr() generated
	// ("cv::Ptr<cv::WarperCreator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::WarperCreator* cv_PtrLcv_WarperCreatorG_getInnerPtr_const(const cv::Ptr<cv::WarperCreator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::WarperCreator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::WarperCreator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::WarperCreator* cv_PtrLcv_WarperCreatorG_getInnerPtrMut(cv::Ptr<cv::WarperCreator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::WarperCreator>::new_null() generated
	// ("cv::Ptr<cv::WarperCreator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::WarperCreator>* cv_PtrLcv_WarperCreatorG_new_null_const() {
			return new cv::Ptr<cv::WarperCreator>();
	}

	// cv::Ptr<cv::WarperCreator>::delete() generated
	// ("cv::Ptr<cv::WarperCreator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_WarperCreatorG_delete(cv::Ptr<cv::WarperCreator>* instance) {
			delete instance;
	}

}

