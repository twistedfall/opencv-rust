extern "C" {
	// cv::Ptr<cv::rgbd::DepthCleaner>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::DepthCleaner* cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtr_const(const cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::DepthCleaner>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::DepthCleaner* cv_PtrLcv_rgbd_DepthCleanerG_getInnerPtrMut(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::DepthCleaner>::new_null() generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::DepthCleaner>* cv_PtrLcv_rgbd_DepthCleanerG_new_null_const() {
			return new cv::Ptr<cv::rgbd::DepthCleaner>();
	}

	// cv::Ptr<cv::rgbd::DepthCleaner>::delete() generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_DepthCleanerG_delete(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::DepthCleaner>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_DepthCleanerG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::DepthCleaner>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::DepthCleaner>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::DepthCleaner>::new", vec![(pred!(const, ["val"], ["cv::rgbd::DepthCleaner"]), _)]),
	cv::Ptr<cv::rgbd::DepthCleaner>* cv_PtrLcv_rgbd_DepthCleanerG_new_const_DepthCleaner(cv::rgbd::DepthCleaner* val) {
			return new cv::Ptr<cv::rgbd::DepthCleaner>(val);
	}

}

