extern "C" {
	// cv::Ptr<cv::rgbd::OdometryFrame>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::OdometryFrame* cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtr_const(const cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::OdometryFrame>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::OdometryFrame* cv_PtrLcv_rgbd_OdometryFrameG_getInnerPtrMut(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::OdometryFrame>::new_null() generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::OdometryFrame>* cv_PtrLcv_rgbd_OdometryFrameG_new_null_const() {
			return new cv::Ptr<cv::rgbd::OdometryFrame>();
	}

	// cv::Ptr<cv::rgbd::OdometryFrame>::delete() generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_OdometryFrameG_delete(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::OdometryFrame>::to_PtrOfRgbdFrame() generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::to_PtrOfRgbdFrame", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdFrame>* cv_PtrLcv_rgbd_OdometryFrameG_to_PtrOfRgbdFrame(cv::Ptr<cv::rgbd::OdometryFrame>* instance) {
			return new cv::Ptr<cv::rgbd::RgbdFrame>(instance->dynamicCast<cv::rgbd::RgbdFrame>());
	}

	// cv::Ptr<cv::rgbd::OdometryFrame>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::OdometryFrame>::new", vec![(pred!(const, ["val"], ["cv::rgbd::OdometryFrame"]), _)]),
	cv::Ptr<cv::rgbd::OdometryFrame>* cv_PtrLcv_rgbd_OdometryFrameG_new_const_OdometryFrame(cv::rgbd::OdometryFrame* val) {
			return new cv::Ptr<cv::rgbd::OdometryFrame>(val);
	}

}

