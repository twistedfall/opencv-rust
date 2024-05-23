extern "C" {
	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::KeypointBasedMotionEstimator* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtr_const(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::KeypointBasedMotionEstimator* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_getInnerPtrMut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::new_null() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_new_null_const() {
			return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>();
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::delete() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::to_PtrOfImageMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::to_PtrOfImageMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
			return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>::new", vec![(pred!(const, ["val"], ["cv::videostab::KeypointBasedMotionEstimator"]), _)]),
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* cv_PtrLcv_videostab_KeypointBasedMotionEstimatorG_new_const_KeypointBasedMotionEstimator(cv::videostab::KeypointBasedMotionEstimator* val) {
			return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>(val);
	}

}

