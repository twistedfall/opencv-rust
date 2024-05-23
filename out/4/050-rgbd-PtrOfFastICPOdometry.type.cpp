extern "C" {
	// cv::Ptr<cv::rgbd::FastICPOdometry>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::FastICPOdometry* cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::FastICPOdometry* cv_PtrLcv_rgbd_FastICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::new_null() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::FastICPOdometry>* cv_PtrLcv_rgbd_FastICPOdometryG_new_null_const() {
			return new cv::Ptr<cv::rgbd::FastICPOdometry>();
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::delete() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_FastICPOdometryG_delete(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::to_PtrOfOdometry() generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::to_PtrOfOdometry", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_FastICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::FastICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}

	// cv::Ptr<cv::rgbd::FastICPOdometry>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::FastICPOdometry>::new", vec![(pred!(const, ["val"], ["cv::rgbd::FastICPOdometry"]), _)]),
	cv::Ptr<cv::rgbd::FastICPOdometry>* cv_PtrLcv_rgbd_FastICPOdometryG_new_const_FastICPOdometry(cv::rgbd::FastICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::FastICPOdometry>(val);
	}

}

