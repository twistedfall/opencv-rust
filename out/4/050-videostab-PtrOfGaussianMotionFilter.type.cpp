extern "C" {
	// cv::Ptr<cv::videostab::GaussianMotionFilter>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::GaussianMotionFilter* cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtr_const(const cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::GaussianMotionFilter* cv_PtrLcv_videostab_GaussianMotionFilterG_getInnerPtrMut(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::new_null() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::GaussianMotionFilter>* cv_PtrLcv_videostab_GaussianMotionFilterG_new_null_const() {
			return new cv::Ptr<cv::videostab::GaussianMotionFilter>();
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::delete() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_GaussianMotionFilterG_delete(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::to_PtrOfIMotionStabilizer() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::to_PtrOfIMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::to_PtrOfMotionFilterBase() generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::to_PtrOfMotionFilterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::MotionFilterBase>* cv_PtrLcv_videostab_GaussianMotionFilterG_to_PtrOfMotionFilterBase(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
			return new cv::Ptr<cv::videostab::MotionFilterBase>(instance->dynamicCast<cv::videostab::MotionFilterBase>());
	}

	// cv::Ptr<cv::videostab::GaussianMotionFilter>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::GaussianMotionFilter>::new", vec![(pred!(const, ["val"], ["cv::videostab::GaussianMotionFilter"]), _)]),
	cv::Ptr<cv::videostab::GaussianMotionFilter>* cv_PtrLcv_videostab_GaussianMotionFilterG_new_const_GaussianMotionFilter(cv::videostab::GaussianMotionFilter* val) {
			return new cv::Ptr<cv::videostab::GaussianMotionFilter>(val);
	}

}

