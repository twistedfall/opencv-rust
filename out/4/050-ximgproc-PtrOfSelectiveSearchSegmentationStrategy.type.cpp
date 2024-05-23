extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

