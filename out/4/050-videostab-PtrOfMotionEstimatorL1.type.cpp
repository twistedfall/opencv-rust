extern "C" {
	// cv::Ptr<cv::videostab::MotionEstimatorL1>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MotionEstimatorL1* cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorL1>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorL1* cv_PtrLcv_videostab_MotionEstimatorL1G_getInnerPtrMut(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorL1>::new_null() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorL1>* cv_PtrLcv_videostab_MotionEstimatorL1G_new_null_const() {
			return new cv::Ptr<cv::videostab::MotionEstimatorL1>();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorL1>::delete() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MotionEstimatorL1G_delete(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MotionEstimatorL1>::to_PtrOfMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::to_PtrOfMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrLcv_videostab_MotionEstimatorL1G_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
			return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::MotionEstimatorL1>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorL1>::new", vec![(pred!(const, ["val"], ["cv::videostab::MotionEstimatorL1"]), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorL1>* cv_PtrLcv_videostab_MotionEstimatorL1G_new_const_MotionEstimatorL1(cv::videostab::MotionEstimatorL1* val) {
			return new cv::Ptr<cv::videostab::MotionEstimatorL1>(val);
	}

}

