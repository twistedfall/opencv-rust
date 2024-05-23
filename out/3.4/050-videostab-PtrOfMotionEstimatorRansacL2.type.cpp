extern "C" {
	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MotionEstimatorRansacL2* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionEstimatorRansacL2* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_getInnerPtrMut(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::new_null() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_new_null_const() {
			return new cv::Ptr<cv::videostab::MotionEstimatorRansacL2>();
	}

	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::delete() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MotionEstimatorRansacL2G_delete(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::to_PtrOfMotionEstimatorBase() generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::to_PtrOfMotionEstimatorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
			return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}

	// cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MotionEstimatorRansacL2>::new", vec![(pred!(const, ["val"], ["cv::videostab::MotionEstimatorRansacL2"]), _)]),
	cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* cv_PtrLcv_videostab_MotionEstimatorRansacL2G_new_const_MotionEstimatorRansacL2(cv::videostab::MotionEstimatorRansacL2* val) {
			return new cv::Ptr<cv::videostab::MotionEstimatorRansacL2>(val);
	}

}

