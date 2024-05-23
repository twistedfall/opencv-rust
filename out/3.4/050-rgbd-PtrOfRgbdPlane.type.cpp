extern "C" {
	// cv::Ptr<cv::rgbd::RgbdPlane>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::RgbdPlane* cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdPlane>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdPlane* cv_PtrLcv_rgbd_RgbdPlaneG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdPlane>::new_null() generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdPlane>* cv_PtrLcv_rgbd_RgbdPlaneG_new_null_const() {
			return new cv::Ptr<cv::rgbd::RgbdPlane>();
	}

	// cv::Ptr<cv::rgbd::RgbdPlane>::delete() generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_RgbdPlaneG_delete(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::RgbdPlane>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdPlaneG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdPlane>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::RgbdPlane>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::RgbdPlane>::new", vec![(pred!(const, ["val"], ["cv::rgbd::RgbdPlane"]), _)]),
	cv::Ptr<cv::rgbd::RgbdPlane>* cv_PtrLcv_rgbd_RgbdPlaneG_new_const_RgbdPlane(cv::rgbd::RgbdPlane* val) {
			return new cv::Ptr<cv::rgbd::RgbdPlane>(val);
	}

}

