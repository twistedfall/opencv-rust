extern "C" {
	// cv::Ptr<cv::kinfu::Volume>::getInnerPtr() generated
	// ("cv::Ptr<cv::kinfu::Volume>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::kinfu::Volume* cv_PtrLcv_kinfu_VolumeG_getInnerPtr_const(const cv::Ptr<cv::kinfu::Volume>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::Volume>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::kinfu::Volume>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::kinfu::Volume* cv_PtrLcv_kinfu_VolumeG_getInnerPtrMut(cv::Ptr<cv::kinfu::Volume>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::Volume>::new_null() generated
	// ("cv::Ptr<cv::kinfu::Volume>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::kinfu::Volume>* cv_PtrLcv_kinfu_VolumeG_new_null_const() {
			return new cv::Ptr<cv::kinfu::Volume>();
	}

	// cv::Ptr<cv::kinfu::Volume>::delete() generated
	// ("cv::Ptr<cv::kinfu::Volume>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_kinfu_VolumeG_delete(cv::Ptr<cv::kinfu::Volume>* instance) {
			delete instance;
	}

}

