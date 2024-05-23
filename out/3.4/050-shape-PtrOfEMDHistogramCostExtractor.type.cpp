extern "C" {
	// cv::Ptr<cv::EMDHistogramCostExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::EMDHistogramCostExtractor* cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::EMDHistogramCostExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::EMDHistogramCostExtractor* cv_PtrLcv_EMDHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::EMDHistogramCostExtractor>::new_null() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::EMDHistogramCostExtractor>* cv_PtrLcv_EMDHistogramCostExtractorG_new_null_const() {
			return new cv::Ptr<cv::EMDHistogramCostExtractor>();
	}

	// cv::Ptr<cv::EMDHistogramCostExtractor>::delete() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_EMDHistogramCostExtractorG_delete(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::EMDHistogramCostExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::EMDHistogramCostExtractor>::to_PtrOfHistogramCostExtractor() generated
	// ("cv::Ptr<cv::EMDHistogramCostExtractor>::to_PtrOfHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_EMDHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}

}

