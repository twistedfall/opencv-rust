extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::to_PtrOfSelectiveSearchSegmentationStrategy() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>::to_PtrOfSelectiveSearchSegmentationStrategy", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationStrategySizeG_to_PtrOfSelectiveSearchSegmentationStrategy(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>* instance) {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>(instance->dynamicCast<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>());
	}

}

