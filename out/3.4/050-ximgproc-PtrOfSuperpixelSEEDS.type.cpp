extern "C" {
	// cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::SuperpixelSEEDS* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SuperpixelSEEDS* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::SuperpixelSEEDS>();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::delete() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_SuperpixelSEEDSG_delete(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSEEDS>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelSEEDSG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelSEEDS>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

