extern "C" {
	// cv::Ptr<cv::xphoto::TonemapDurand>::getInnerPtr() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xphoto::TonemapDurand* cv_PtrLcv_xphoto_TonemapDurandG_getInnerPtr_const(const cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::TonemapDurand>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::TonemapDurand* cv_PtrLcv_xphoto_TonemapDurandG_getInnerPtrMut(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::TonemapDurand>::new_null() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xphoto::TonemapDurand>* cv_PtrLcv_xphoto_TonemapDurandG_new_null_const() {
			return new cv::Ptr<cv::xphoto::TonemapDurand>();
	}

	// cv::Ptr<cv::xphoto::TonemapDurand>::delete() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xphoto_TonemapDurandG_delete(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xphoto::TonemapDurand>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xphoto_TonemapDurandG_to_PtrOfAlgorithm(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xphoto::TonemapDurand>::to_PtrOfTonemap() generated
	// ("cv::Ptr<cv::xphoto::TonemapDurand>::to_PtrOfTonemap", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Tonemap>* cv_PtrLcv_xphoto_TonemapDurandG_to_PtrOfTonemap(cv::Ptr<cv::xphoto::TonemapDurand>* instance) {
			return new cv::Ptr<cv::Tonemap>(instance->dynamicCast<cv::Tonemap>());
	}

}

