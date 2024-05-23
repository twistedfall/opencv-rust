extern "C" {
	// cv::Ptr<cv::ximgproc::SuperpixelLSC>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelLSC>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::SuperpixelLSC* cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelLSC>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelLSC>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::SuperpixelLSC* cv_PtrLcv_ximgproc_SuperpixelLSCG_getInnerPtrMut(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelLSC>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelLSC>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::SuperpixelLSC>* cv_PtrLcv_ximgproc_SuperpixelLSCG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::SuperpixelLSC>();
	}

	// cv::Ptr<cv::ximgproc::SuperpixelLSC>::delete() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelLSC>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_SuperpixelLSCG_delete(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::SuperpixelLSC>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::SuperpixelLSC>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_SuperpixelLSCG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::SuperpixelLSC>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

