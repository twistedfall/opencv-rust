extern "C" {
	// cv::Ptr<cv::rgbd::ICPOdometry>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::ICPOdometry* cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::ICPOdometry* cv_PtrLcv_rgbd_ICPOdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::new_null() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::ICPOdometry>* cv_PtrLcv_rgbd_ICPOdometryG_new_null_const() {
			return new cv::Ptr<cv::rgbd::ICPOdometry>();
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::delete() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_ICPOdometryG_delete(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::to_PtrOfOdometry() generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::to_PtrOfOdometry", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_ICPOdometryG_to_PtrOfOdometry(cv::Ptr<cv::rgbd::ICPOdometry>* instance) {
			return new cv::Ptr<cv::rgbd::Odometry>(instance->dynamicCast<cv::rgbd::Odometry>());
	}

	// cv::Ptr<cv::rgbd::ICPOdometry>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::ICPOdometry>::new", vec![(pred!(const, ["val"], ["cv::rgbd::ICPOdometry"]), _)]),
	cv::Ptr<cv::rgbd::ICPOdometry>* cv_PtrLcv_rgbd_ICPOdometryG_new_const_ICPOdometry(cv::rgbd::ICPOdometry* val) {
			return new cv::Ptr<cv::rgbd::ICPOdometry>(val);
	}

}

