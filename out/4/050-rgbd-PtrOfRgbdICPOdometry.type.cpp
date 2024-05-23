extern "C" {
	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::RgbdICPOdometry* cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdICPOdometry* cv_PtrLcv_rgbd_RgbdICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::new_null() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdICPOdometry>* cv_PtrLcv_rgbd_RgbdICPOdometryG_new_null_const() {
			return new cv::Ptr<cv::rgbd::RgbdICPOdometry>();
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::delete() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_RgbdICPOdometryG_delete(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::to_PtrOfOdometry() generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::to_PtrOfOdometry", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_RgbdICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::RgbdICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}

	// cv::Ptr<cv::rgbd::RgbdICPOdometry>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::RgbdICPOdometry>::new", vec![(pred!(const, ["val"], ["cv::rgbd::RgbdICPOdometry"]), _)]),
	cv::Ptr<cv::rgbd::RgbdICPOdometry>* cv_PtrLcv_rgbd_RgbdICPOdometryG_new_const_RgbdICPOdometry(cv::rgbd::RgbdICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::RgbdICPOdometry>(val);
	}

}

