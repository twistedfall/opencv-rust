extern "C" {
	// cv::Ptr<cv::FileStorage>::getInnerPtr() generated
	// ("cv::Ptr<cv::FileStorage>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::FileStorage* cv_PtrLcv_FileStorageG_getInnerPtr_const(const cv::Ptr<cv::FileStorage>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FileStorage>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::FileStorage>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::FileStorage* cv_PtrLcv_FileStorageG_getInnerPtrMut(cv::Ptr<cv::FileStorage>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::FileStorage>::new_null() generated
	// ("cv::Ptr<cv::FileStorage>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::FileStorage>* cv_PtrLcv_FileStorageG_new_null_const() {
			return new cv::Ptr<cv::FileStorage>();
	}

	// cv::Ptr<cv::FileStorage>::delete() generated
	// ("cv::Ptr<cv::FileStorage>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_FileStorageG_delete(cv::Ptr<cv::FileStorage>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::FileStorage>::new(TraitClass) generated
	// ("cv::Ptr<cv::FileStorage>::new", vec![(pred!(const, ["val"], ["cv::FileStorage"]), _)]),
	cv::Ptr<cv::FileStorage>* cv_PtrLcv_FileStorageG_new_const_FileStorage(cv::FileStorage* val) {
			return new cv::Ptr<cv::FileStorage>(val);
	}

}

