extern "C" {
	// cv::Ptr<cv::videostab::TwoPassStabilizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::TwoPassStabilizer* cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::TwoPassStabilizer* cv_PtrLcv_videostab_TwoPassStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::new_null() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::TwoPassStabilizer>* cv_PtrLcv_videostab_TwoPassStabilizerG_new_null_const() {
			return new cv::Ptr<cv::videostab::TwoPassStabilizer>();
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::delete() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_TwoPassStabilizerG_delete(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::to_PtrOfIFrameSource() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::to_PtrOfIFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::to_PtrOfStabilizerBase() generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::to_PtrOfStabilizerBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::StabilizerBase>* cv_PtrLcv_videostab_TwoPassStabilizerG_to_PtrOfStabilizerBase(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::StabilizerBase>(instance->dynamicCast<cv::videostab::StabilizerBase>());
	}

	// cv::Ptr<cv::videostab::TwoPassStabilizer>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::TwoPassStabilizer>::new", vec![(pred!(const, ["val"], ["cv::videostab::TwoPassStabilizer"]), _)]),
	cv::Ptr<cv::videostab::TwoPassStabilizer>* cv_PtrLcv_videostab_TwoPassStabilizerG_new_const_TwoPassStabilizer(cv::videostab::TwoPassStabilizer* val) {
			return new cv::Ptr<cv::videostab::TwoPassStabilizer>(val);
	}

}

