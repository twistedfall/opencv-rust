extern "C" {
	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::EMDL1HistogramCostExtractor* cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::EMDL1HistogramCostExtractor* cv_PtrLcv_EMDL1HistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::new_null() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::EMDL1HistogramCostExtractor>* cv_PtrLcv_EMDL1HistogramCostExtractorG_new_null_const() {
			return new cv::Ptr<cv::EMDL1HistogramCostExtractor>();
	}

	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::delete() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_EMDL1HistogramCostExtractorG_delete(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::EMDL1HistogramCostExtractor>::to_PtrOfHistogramCostExtractor() generated
	// ("cv::Ptr<cv::EMDL1HistogramCostExtractor>::to_PtrOfHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_EMDL1HistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::EMDL1HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}

}

