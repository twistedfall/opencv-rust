extern "C" {
	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::KeypointBasedMotionEstimatorGpu* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_getInnerPtr_const(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::KeypointBasedMotionEstimatorGpu* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_getInnerPtrMut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::new_null() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_new_null_const() {
			return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::delete() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::to_PtrOfImageMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::to_PtrOfImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>::new", vec![(pred!(const, ["val"], ["cv::videostab::KeypointBasedMotionEstimatorGpu"]), _)]),
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorGpuG_new_const_KeypointBasedMotionEstimatorGpu(cv::videostab::KeypointBasedMotionEstimatorGpu* val) {
			return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimatorGpu>(val);
	}

}

