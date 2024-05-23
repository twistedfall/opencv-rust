extern "C" {
	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::getInnerPtr() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtr_const(const cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ximgproc::segmentation::SelectiveSearchSegmentation* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_getInnerPtrMut(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::new_null() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_new_null_const() {
			return new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>();
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::delete() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_delete(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ximgproc_segmentation_SelectiveSearchSegmentationG_to_PtrOfAlgorithm(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

