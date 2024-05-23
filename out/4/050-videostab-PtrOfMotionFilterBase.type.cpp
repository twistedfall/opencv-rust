extern "C" {
	// cv::Ptr<cv::videostab::MotionFilterBase>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MotionFilterBase>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MotionFilterBase* cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtr_const(const cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionFilterBase>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MotionFilterBase>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MotionFilterBase* cv_PtrLcv_videostab_MotionFilterBaseG_getInnerPtrMut(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MotionFilterBase>::new_null() generated
	// ("cv::Ptr<cv::videostab::MotionFilterBase>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MotionFilterBase>* cv_PtrLcv_videostab_MotionFilterBaseG_new_null_const() {
			return new cv::Ptr<cv::videostab::MotionFilterBase>();
	}

	// cv::Ptr<cv::videostab::MotionFilterBase>::delete() generated
	// ("cv::Ptr<cv::videostab::MotionFilterBase>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MotionFilterBaseG_delete(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MotionFilterBase>::to_PtrOfIMotionStabilizer() generated
	// ("cv::Ptr<cv::videostab::MotionFilterBase>::to_PtrOfIMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_MotionFilterBaseG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}

}

