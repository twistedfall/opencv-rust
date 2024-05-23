extern "C" {
	// cv::Ptr<cv::ShapeDistanceExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::ShapeDistanceExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ShapeDistanceExtractor* cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeDistanceExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ShapeDistanceExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ShapeDistanceExtractor* cv_PtrLcv_ShapeDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeDistanceExtractor>::new_null() generated
	// ("cv::Ptr<cv::ShapeDistanceExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ShapeDistanceExtractor>* cv_PtrLcv_ShapeDistanceExtractorG_new_null_const() {
			return new cv::Ptr<cv::ShapeDistanceExtractor>();
	}

	// cv::Ptr<cv::ShapeDistanceExtractor>::delete() generated
	// ("cv::Ptr<cv::ShapeDistanceExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ShapeDistanceExtractorG_delete(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ShapeDistanceExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ShapeDistanceExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

