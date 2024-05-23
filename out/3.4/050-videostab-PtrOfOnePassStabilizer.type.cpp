extern "C" {
	// cv::Ptr<cv::videostab::OnePassStabilizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::OnePassStabilizer* cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::OnePassStabilizer* cv_PtrLcv_videostab_OnePassStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::new_null() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::OnePassStabilizer>* cv_PtrLcv_videostab_OnePassStabilizerG_new_null_const() {
			return new cv::Ptr<cv::videostab::OnePassStabilizer>();
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::delete() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_OnePassStabilizerG_delete(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::to_PtrOfIFrameSource() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::to_PtrOfIFrameSource", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::to_PtrOfStabilizerBase() generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::to_PtrOfStabilizerBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::StabilizerBase>* cv_PtrLcv_videostab_OnePassStabilizerG_to_PtrOfStabilizerBase(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::StabilizerBase>(instance->dynamicCast<cv::videostab::StabilizerBase>());
	}

	// cv::Ptr<cv::videostab::OnePassStabilizer>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::OnePassStabilizer>::new", vec![(pred!(const, ["val"], ["cv::videostab::OnePassStabilizer"]), _)]),
	cv::Ptr<cv::videostab::OnePassStabilizer>* cv_PtrLcv_videostab_OnePassStabilizerG_new_const_OnePassStabilizer(cv::videostab::OnePassStabilizer* val) {
			return new cv::Ptr<cv::videostab::OnePassStabilizer>(val);
	}

}

