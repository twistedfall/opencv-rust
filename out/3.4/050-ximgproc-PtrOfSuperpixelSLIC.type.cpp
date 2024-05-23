extern "C" {
	// cv::Ptr<cv::ximgproc::SuperpixelSLIC>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSLIC>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::SuperpixelSLIC* cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSLIC>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSLIC>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SuperpixelSLIC* cv_PtrLcv_ximgproc_SuperpixelSLICG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSLIC>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSLIC>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::SuperpixelSLIC>* cv_PtrLcv_ximgproc_SuperpixelSLICG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::SuperpixelSLIC>();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSLIC>::delete() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSLIC>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_SuperpixelSLICG_delete(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::SuperpixelSLIC>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelSLIC>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelSLICG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelSLIC>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

