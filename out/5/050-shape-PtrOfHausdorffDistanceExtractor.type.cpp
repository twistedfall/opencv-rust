extern "C" {
	// cv::Ptr<cv::HausdorffDistanceExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::HausdorffDistanceExtractor* cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtr_const(const cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::HausdorffDistanceExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::HausdorffDistanceExtractor* cv_PtrLcv_HausdorffDistanceExtractorG_getInnerPtrMut(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::HausdorffDistanceExtractor>::new_null() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::HausdorffDistanceExtractor>* cv_PtrLcv_HausdorffDistanceExtractorG_new_null_const() {
			return new cv::Ptr<cv::HausdorffDistanceExtractor>();
	}

	// cv::Ptr<cv::HausdorffDistanceExtractor>::delete() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_HausdorffDistanceExtractorG_delete(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::HausdorffDistanceExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::HausdorffDistanceExtractor>::to_PtrOfShapeDistanceExtractor() generated
	// ("cv::Ptr<cv::HausdorffDistanceExtractor>::to_PtrOfShapeDistanceExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::ShapeDistanceExtractor>* cv_PtrLcv_HausdorffDistanceExtractorG_to_PtrOfShapeDistanceExtractor(cv::Ptr<cv::HausdorffDistanceExtractor>* instance) {
			return new cv::Ptr<cv::ShapeDistanceExtractor>(instance->dynamicCast<cv::ShapeDistanceExtractor>());
	}

}

