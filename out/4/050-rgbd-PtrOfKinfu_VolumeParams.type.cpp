extern "C" {
	// cv::Ptr<cv::kinfu::VolumeParams>::getInnerPtr() generated
	// ("cv::Ptr<cv::kinfu::VolumeParams>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::kinfu::VolumeParams* cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtr_const(const cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::VolumeParams>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::kinfu::VolumeParams>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::kinfu::VolumeParams* cv_PtrLcv_kinfu_VolumeParamsG_getInnerPtrMut(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::VolumeParams>::new_null() generated
	// ("cv::Ptr<cv::kinfu::VolumeParams>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::kinfu::VolumeParams>* cv_PtrLcv_kinfu_VolumeParamsG_new_null_const() {
			return new cv::Ptr<cv::kinfu::VolumeParams>();
	}

	// cv::Ptr<cv::kinfu::VolumeParams>::delete() generated
	// ("cv::Ptr<cv::kinfu::VolumeParams>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_kinfu_VolumeParamsG_delete(cv::Ptr<cv::kinfu::VolumeParams>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::kinfu::VolumeParams>::new(TraitClass) generated
	// ("cv::Ptr<cv::kinfu::VolumeParams>::new", vec![(pred!(const, ["val"], ["cv::kinfu::VolumeParams"]), _)]),
	cv::Ptr<cv::kinfu::VolumeParams>* cv_PtrLcv_kinfu_VolumeParamsG_new_const_VolumeParams(cv::kinfu::VolumeParams* val) {
			return new cv::Ptr<cv::kinfu::VolumeParams>(val);
	}

}

