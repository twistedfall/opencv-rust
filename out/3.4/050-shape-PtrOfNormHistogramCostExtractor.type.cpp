extern "C" {
	// cv::Ptr<cv::NormHistogramCostExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::NormHistogramCostExtractor* cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::NormHistogramCostExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::NormHistogramCostExtractor* cv_PtrLcv_NormHistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::NormHistogramCostExtractor>::new_null() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::NormHistogramCostExtractor>* cv_PtrLcv_NormHistogramCostExtractorG_new_null_const() {
			return new cv::Ptr<cv::NormHistogramCostExtractor>();
	}

	// cv::Ptr<cv::NormHistogramCostExtractor>::delete() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_NormHistogramCostExtractorG_delete(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::NormHistogramCostExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::NormHistogramCostExtractor>::to_PtrOfHistogramCostExtractor() generated
	// ("cv::Ptr<cv::NormHistogramCostExtractor>::to_PtrOfHistogramCostExtractor", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_NormHistogramCostExtractorG_to_PtrOfHistogramCostExtractor(cv::Ptr<cv::NormHistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::HistogramCostExtractor>(instance->dynamicCast<cv::HistogramCostExtractor>());
	}

}

