extern "C" {
	// cv::Ptr<cv::detail::GraphCutSeamFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::GraphCutSeamFinder* cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinder* cv_PtrLcv_detail_GraphCutSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::GraphCutSeamFinder>();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::delete() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_GraphCutSeamFinderG_delete(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::to_PtrOfDetail_GraphCutSeamFinderBase() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::to_PtrOfDetail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinderBase>* cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_GraphCutSeamFinderBase(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinderBase>(instance->dynamicCast<cv::detail::GraphCutSeamFinderBase>());
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::GraphCutSeamFinder"]), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderG_new_const_GraphCutSeamFinder(cv::detail::GraphCutSeamFinder* val) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinder>(val);
	}

}

