extern "C" {
	// cv::Ptr<cv::rgbd::Odometry>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::Odometry>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::Odometry* cv_PtrLcv_rgbd_OdometryG_getInnerPtr_const(const cv::Ptr<cv::rgbd::Odometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::Odometry>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::Odometry>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::Odometry* cv_PtrLcv_rgbd_OdometryG_getInnerPtrMut(cv::Ptr<cv::rgbd::Odometry>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::Odometry>::new_null() generated
	// ("cv::Ptr<cv::rgbd::Odometry>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::Odometry>* cv_PtrLcv_rgbd_OdometryG_new_null_const() {
			return new cv::Ptr<cv::rgbd::Odometry>();
	}

	// cv::Ptr<cv::rgbd::Odometry>::delete() generated
	// ("cv::Ptr<cv::rgbd::Odometry>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_OdometryG_delete(cv::Ptr<cv::rgbd::Odometry>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::Odometry>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::Odometry>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_OdometryG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::Odometry>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

