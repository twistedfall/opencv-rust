extern "C" {
	// cv::Ptr<cv::ShapeContextDistanceExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ShapeContextDistanceExtractor* cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeContextDistanceExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ShapeContextDistanceExtractor* cv_PtrLcv_ShapeContextDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ShapeContextDistanceExtractor>::new_null() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ShapeContextDistanceExtractor>* cv_PtrLcv_ShapeContextDistanceExtractorG_new_null_const() {
			return new cv::Ptr<cv::ShapeContextDistanceExtractor>();
	}

	// cv::Ptr<cv::ShapeContextDistanceExtractor>::delete() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ShapeContextDistanceExtractorG_delete(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ShapeContextDistanceExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ShapeContextDistanceExtractor>::to_PtrOfShapeDistanceExtractor() generated
	// ("cv::Ptr<cv::ShapeContextDistanceExtractor>::to_PtrOfShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ShapeDistanceExtractor>* cv_PtrLcv_ShapeContextDistanceExtractorG_to_PtrOfShapeDistanceExtractor(cv::Ptr<cv::ShapeContextDistanceExtractor>* instance) {
			return new cv::Ptr<cv::ShapeDistanceExtractor>(instance->dynamicCast<cv::ShapeDistanceExtractor>());
	}

}

