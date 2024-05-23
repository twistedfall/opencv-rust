extern "C" {
	// cv::Ptr<cv::videostab::LpMotionStabilizer>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::LpMotionStabilizer* cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtr_const(const cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::LpMotionStabilizer>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::LpMotionStabilizer* cv_PtrLcv_videostab_LpMotionStabilizerG_getInnerPtrMut(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::LpMotionStabilizer>::new_null() generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::LpMotionStabilizer>* cv_PtrLcv_videostab_LpMotionStabilizerG_new_null_const() {
			return new cv::Ptr<cv::videostab::LpMotionStabilizer>();
	}

	// cv::Ptr<cv::videostab::LpMotionStabilizer>::delete() generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_LpMotionStabilizerG_delete(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::LpMotionStabilizer>::to_PtrOfIMotionStabilizer() generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::to_PtrOfIMotionStabilizer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrLcv_videostab_LpMotionStabilizerG_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
			return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}

	// cv::Ptr<cv::videostab::LpMotionStabilizer>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::LpMotionStabilizer>::new", vec![(pred!(const, ["val"], ["cv::videostab::LpMotionStabilizer"]), _)]),
	cv::Ptr<cv::videostab::LpMotionStabilizer>* cv_PtrLcv_videostab_LpMotionStabilizerG_new_const_LpMotionStabilizer(cv::videostab::LpMotionStabilizer* val) {
			return new cv::Ptr<cv::videostab::LpMotionStabilizer>(val);
	}

}

