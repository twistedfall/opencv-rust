extern "C" {
	// cv::Ptr<cv::rgbd::RgbdOdometry>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::RgbdOdometry* cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdOdometry* cv_PtrLcv_rgbd_RgbdOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::new_null() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdOdometry>* cv_PtrLcv_rgbd_RgbdOdometryG_new_null_const() {
			return new cv::Ptr<cv::rgbd::RgbdOdometry>();
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::delete() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_RgbdOdometryG_delete(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::to_PtrOfOdometry() generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::to_PtrOfOdometry", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_RgbdOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::RgbdOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}

	// cv::Ptr<cv::rgbd::RgbdOdometry>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::RgbdOdometry>::new", vec![(pred!(const, ["val"], ["cv::rgbd::RgbdOdometry"]), _)]),
	cv::Ptr<cv::rgbd::RgbdOdometry>* cv_PtrLcv_rgbd_RgbdOdometryG_new_const_RgbdOdometry(cv::rgbd::RgbdOdometry* val) {
			return new cv::Ptr<cv::rgbd::RgbdOdometry>(val);
	}

}

