extern "C" {
	// cv::Ptr<cv::ChiHistogramCostExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::ChiHistogramCostExtractor* cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ChiHistogramCostExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::ChiHistogramCostExtractor* cv_PtrLcv_ChiHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::ChiHistogramCostExtractor>::new_null() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::ChiHistogramCostExtractor>* cv_PtrLcv_ChiHistogramCostExtractorG_new_null_const() {
			return new cv::Ptr<cv::ChiHistogramCostExtractor>();
	}

	// cv::Ptr<cv::ChiHistogramCostExtractor>::delete() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_ChiHistogramCostExtractorG_delete(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::ChiHistogramCostExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::ChiHistogramCostExtractor>::to_PtrOfHistogramCostExtractor() generated
	// ("cv::Ptr<cv::ChiHistogramCostExtractor>::to_PtrOfHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_ChiHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::ChiHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}

}

