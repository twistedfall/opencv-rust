extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::to_PtrOfSelectiveSearchSegmentationStrategy() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>::to_PtrOfSelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyColorG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}

}

