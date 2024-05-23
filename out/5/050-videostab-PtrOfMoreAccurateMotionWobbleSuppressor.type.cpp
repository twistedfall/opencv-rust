extern "C" {
	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtr_const(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressor* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_getInnerPtrMut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::new_null() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_new_null_const() {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::delete() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::to_PtrOfMoreAccurateMotionWobbleSuppressorBase() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::to_PtrOfMoreAccurateMotionWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfMoreAccurateMotionWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>(instance->dynamicCast<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>());
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::to_PtrOfWobbleSuppressorBase() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::to_PtrOfWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::new(TraitClass) generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>::new", vec![(pred!(const, ["val"], ["cv::videostab::MoreAccurateMotionWobbleSuppressor"]), _)]),
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorG_new_const_MoreAccurateMotionWobbleSuppressor(cv::videostab::MoreAccurateMotionWobbleSuppressor* val) {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressor>(val);
	}

}

