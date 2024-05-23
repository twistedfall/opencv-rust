extern "C" {
	// cv::Ptr<cv::xphoto::SimpleWB>::getInnerPtr() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xphoto::SimpleWB* cv_PtrLcv_xphoto_SimpleWBG_getInnerPtr_const(const cv::Ptr<cv::xphoto::SimpleWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::SimpleWB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::SimpleWB* cv_PtrLcv_xphoto_SimpleWBG_getInnerPtrMut(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::SimpleWB>::new_null() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xphoto::SimpleWB>* cv_PtrLcv_xphoto_SimpleWBG_new_null_const() {
			return new cv::Ptr<cv::xphoto::SimpleWB>();
	}

	// cv::Ptr<cv::xphoto::SimpleWB>::delete() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xphoto_SimpleWBG_delete(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xphoto::SimpleWB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xphoto_SimpleWBG_to_PtrOfAlgorithm(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xphoto::SimpleWB>::to_PtrOfWhiteBalancer() generated
	// ("cv::Ptr<cv::xphoto::SimpleWB>::to_PtrOfWhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::xphoto::WhiteBalancer>* cv_PtrLcv_xphoto_SimpleWBG_to_PtrOfWhiteBalancer(cv::Ptr<cv::xphoto::SimpleWB>* instance) {
			return new cv::Ptr<cv::xphoto::WhiteBalancer>(instance->dynamicCast<cv::xphoto::WhiteBalancer>());
	}

}

