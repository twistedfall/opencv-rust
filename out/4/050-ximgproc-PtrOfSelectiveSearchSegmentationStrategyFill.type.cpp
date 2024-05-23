extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::to_PtrOfSelectiveSearchSegmentationStrategy() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>::to_PtrOfSelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyFillG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}

}

