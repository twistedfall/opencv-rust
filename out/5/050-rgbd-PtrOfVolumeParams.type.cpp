extern "C" {
	// cv::Ptr<cv::large_kinfu::VolumeParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::large_kinfu::VolumeParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::large_kinfu::VolumeParams* cv_PtrLcv_large_kinfu_VolumeParamsG_getInnerPtr_const(const cv::Ptr<cv::large_kinfu::VolumeParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::large_kinfu::VolumeParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::large_kinfu::VolumeParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::large_kinfu::VolumeParams* cv_PtrLcv_large_kinfu_VolumeParamsG_getInnerPtrMut(cv::Ptr<cv::large_kinfu::VolumeParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::large_kinfu::VolumeParams>::new_null() generated
	// ("cv::Ptr<cv::large_kinfu::VolumeParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::large_kinfu::VolumeParams>* cv_PtrLcv_large_kinfu_VolumeParamsG_new_null_const() {
			return new cv::Ptr<cv::large_kinfu::VolumeParams>();
	}

	// cv::Ptr<cv::large_kinfu::VolumeParams>::delete() generated
	// ("cv::Ptr<cv::large_kinfu::VolumeParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_large_kinfu_VolumeParamsG_delete(cv::Ptr<cv::large_kinfu::VolumeParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::large_kinfu::VolumeParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::large_kinfu::VolumeParams>::new", vec![(pred!(const, ["val"], ["cv::large_kinfu::VolumeParams"]), _)]),
	cv::Ptr<cv::large_kinfu::VolumeParams>* cv_PtrLcv_large_kinfu_VolumeParamsG_new_const_VolumeParams(cv::large_kinfu::VolumeParams* val) {
			return new cv::Ptr<cv::large_kinfu::VolumeParams>(val);
	}

}

