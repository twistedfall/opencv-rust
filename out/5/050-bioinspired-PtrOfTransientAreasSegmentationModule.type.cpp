extern "C" {
	// cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::getInnerPtr() generated
	// ("cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::bioinspired::TransientAreasSegmentationModule* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtr_const(const cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::bioinspired::TransientAreasSegmentationModule* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_getInnerPtrMut(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::new_null() generated
	// ("cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_new_null_const() {
			return new cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>();
	}

	// cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::delete() generated
	// ("cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_delete(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_bioinspired_TransientAreasSegmentationModuleG_to_PtrOfAlgorithm(cv::Ptr<cv::bioinspired::TransientAreasSegmentationModule>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

