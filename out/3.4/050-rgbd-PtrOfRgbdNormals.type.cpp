extern "C" {
	// cv::Ptr<cv::rgbd::RgbdNormals>::getInnerPtr() generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::rgbd::RgbdNormals* cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtr_const(const cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdNormals>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::rgbd::RgbdNormals* cv_PtrLcv_rgbd_RgbdNormalsG_getInnerPtrMut(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::rgbd::RgbdNormals>::new_null() generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::rgbd::RgbdNormals>* cv_PtrLcv_rgbd_RgbdNormalsG_new_null_const() {
			return new cv::Ptr<cv::rgbd::RgbdNormals>();
	}

	// cv::Ptr<cv::rgbd::RgbdNormals>::delete() generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_rgbd_RgbdNormalsG_delete(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::rgbd::RgbdNormals>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_rgbd_RgbdNormalsG_to_PtrOfAlgorithm(cv::Ptr<cv::rgbd::RgbdNormals>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::rgbd::RgbdNormals>::new(TraitClass) generated
	// ("cv::Ptr<cv::rgbd::RgbdNormals>::new", vec![(pred!(const, ["val"], ["cv::rgbd::RgbdNormals"]), _)]),
	cv::Ptr<cv::rgbd::RgbdNormals>* cv_PtrLcv_rgbd_RgbdNormalsG_new_const_RgbdNormals(cv::rgbd::RgbdNormals* val) {
			return new cv::Ptr<cv::rgbd::RgbdNormals>(val);
	}

}

