extern "C" {
	// cv::Ptr<cv::HistogramCostExtractor>::getInnerPtr() generated
	// ("cv::Ptr<cv::HistogramCostExtractor>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::HistogramCostExtractor* cv_PtrLcv_HistogramCostExtractorG_getInnerPtr_const(const cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::HistogramCostExtractor>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::HistogramCostExtractor>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::HistogramCostExtractor* cv_PtrLcv_HistogramCostExtractorG_getInnerPtrMut(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::HistogramCostExtractor>::new_null() generated
	// ("cv::Ptr<cv::HistogramCostExtractor>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::HistogramCostExtractor>* cv_PtrLcv_HistogramCostExtractorG_new_null_const() {
			return new cv::Ptr<cv::HistogramCostExtractor>();
	}

	// cv::Ptr<cv::HistogramCostExtractor>::delete() generated
	// ("cv::Ptr<cv::HistogramCostExtractor>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_HistogramCostExtractorG_delete(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::HistogramCostExtractor>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::HistogramCostExtractor>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_HistogramCostExtractorG_to_PtrOfAlgorithm(cv::Ptr<cv::HistogramCostExtractor>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

}

