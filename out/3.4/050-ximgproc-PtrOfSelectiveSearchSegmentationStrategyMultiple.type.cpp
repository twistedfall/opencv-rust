extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::to_PtrOfSelectiveSearchSegmentationStrategy() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>::to_PtrOfSelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultipleG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}

}

