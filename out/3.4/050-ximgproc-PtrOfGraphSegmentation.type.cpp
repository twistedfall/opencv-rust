extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::GraphSegmentation* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::GraphSegmentation* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_delete(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_GraphSegmentationG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

