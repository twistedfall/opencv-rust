extern "C" {
	// cv::Ptr<cv::rgbd::RgbdFrame>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::RgbdFrame>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::RgbdFrame* cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdFrame>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::RgbdFrame>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdFrame* cv_PtrLcv_rgbd_RgbdFrameG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdFrame>::new_null() generated
	// ("cv::Ptr<cv::rgbd::RgbdFrame>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrLcv_rgbd_RgbdFrameG_new_null_const() {
			return new cv::Ptr<cv::rgbd::RgbdFrame>();
	}

	// cv::Ptr<cv::rgbd::RgbdFrame>::delete() generated
	// ("cv::Ptr<cv::rgbd::RgbdFrame>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_RgbdFrameG_delete(cv::Ptr<cv::rgbd::RgbdFrame>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::RgbdFrame>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::RgbdFrame>::new", vec![(pred!(const, ["val"], ["cv::rgbd::RgbdFrame"]), _)]),
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrLcv_rgbd_RgbdFrameG_new_const_RgbdFrame(cv::rgbd::RgbdFrame* val) {
			return new cv::Ptr<cv::rgbd::RgbdFrame>(val);
	}

}

