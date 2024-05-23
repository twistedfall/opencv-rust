extern "C" {
	// cv::Ptr<cv::xphoto::LearningBasedWB>::getInnerPtr() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::xphoto::LearningBasedWB* cv_PtrLcv_xphoto_LearningBasedWBG_getInnerPtr_const(const cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::LearningBasedWB>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::xphoto::LearningBasedWB* cv_PtrLcv_xphoto_LearningBasedWBG_getInnerPtrMut(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::xphoto::LearningBasedWB>::new_null() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::xphoto::LearningBasedWB>* cv_PtrLcv_xphoto_LearningBasedWBG_new_null_const() {
			return new cv::Ptr<cv::xphoto::LearningBasedWB>();
	}

	// cv::Ptr<cv::xphoto::LearningBasedWB>::delete() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_xphoto_LearningBasedWBG_delete(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::xphoto::LearningBasedWB>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_xphoto_LearningBasedWBG_to_PtrOfAlgorithm(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::xphoto::LearningBasedWB>::to_PtrOfWhiteBalancer() generated
	// ("cv::Ptr<cv::xphoto::LearningBasedWB>::to_PtrOfWhiteBalancer", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::xphoto::WhiteBalancer>* cv_PtrLcv_xphoto_LearningBasedWBG_to_PtrOfWhiteBalancer(cv::Ptr<cv::xphoto::LearningBasedWB>* instance) {
			return new cv::Ptr<cv::xphoto::WhiteBalancer>(instance->dynamicCast<cv::xphoto::WhiteBalancer>());
	}

}

