extern "C" {
	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::getInnerPtr() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtr_const(const cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::videostab::MoreAccurateMotionWobbleSuppressorBase* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_getInnerPtrMut(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::new_null() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_new_null_const() {
			return new cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>();
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::delete() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_delete(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::to_PtrOfWobbleSuppressorBase() generated
	// ("cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>::to_PtrOfWobbleSuppressorBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::videostab::WobbleSuppressorBase>* cv_PtrLcv_videostab_MoreAccurateMotionWobbleSuppressorBaseG_to_PtrOfWobbleSuppressorBase(cv::Ptr<cv::videostab::MoreAccurateMotionWobbleSuppressorBase>* instance) {
			return new cv::Ptr<cv::videostab::WobbleSuppressorBase>(instance->dynamicCast<cv::videostab::WobbleSuppressorBase>());
	}

}

