extern "C" {
	// cv::Ptr<cv::xphoto::GrayworldWB>::getInnerPtr() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xphoto::GrayworldWB* cv_PtrLcv_xphoto_GrayworldWBG_getInnerPtr_const(const cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::GrayworldWB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::GrayworldWB* cv_PtrLcv_xphoto_GrayworldWBG_getInnerPtrMut(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::GrayworldWB>::new_null() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xphoto::GrayworldWB>* cv_PtrLcv_xphoto_GrayworldWBG_new_null_const() {
			return new cv::Ptr<cv::xphoto::GrayworldWB>();
	}

	// cv::Ptr<cv::xphoto::GrayworldWB>::delete() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xphoto_GrayworldWBG_delete(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xphoto::GrayworldWB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xphoto_GrayworldWBG_to_PtrOfAlgorithm(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xphoto::GrayworldWB>::to_PtrOfWhiteBalancer() generated
	// ("cv::Ptr<cv::xphoto::GrayworldWB>::to_PtrOfWhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::xphoto::WhiteBalancer>* cv_PtrLcv_xphoto_GrayworldWBG_to_PtrOfWhiteBalancer(cv::Ptr<cv::xphoto::GrayworldWB>* instance) {
			return new cv::Ptr<cv::xphoto::WhiteBalancer>(instance->dynamicCast<cv::xphoto::WhiteBalancer>());
	}

}

