extern "C" {
	// cv::Ptr<cv::ximgproc::RFFeatureGetter>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::RFFeatureGetter>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::RFFeatureGetter* cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RFFeatureGetter>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::RFFeatureGetter>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::RFFeatureGetter* cv_PtrLcv_ximgproc_RFFeatureGetterG_getInnerPtrMut(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::RFFeatureGetter>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::RFFeatureGetter>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::RFFeatureGetter>* cv_PtrLcv_ximgproc_RFFeatureGetterG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::RFFeatureGetter>();
	}

	// cv::Ptr<cv::ximgproc::RFFeatureGetter>::delete() generated
	// ("cv::Ptr<cv::ximgproc::RFFeatureGetter>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_RFFeatureGetterG_delete(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::RFFeatureGetter>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::RFFeatureGetter>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_RFFeatureGetterG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::RFFeatureGetter>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

