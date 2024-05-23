extern "C" {
	// cv::Ptr<cv::detail::PairwiseSeamFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::PairwiseSeamFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::PairwiseSeamFinder* cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::PairwiseSeamFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::PairwiseSeamFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::PairwiseSeamFinder* cv_PtrLcv_detail_PairwiseSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::PairwiseSeamFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::PairwiseSeamFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::PairwiseSeamFinder>* cv_PtrLcv_detail_PairwiseSeamFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::PairwiseSeamFinder>();
	}

	// cv::Ptr<cv::detail::PairwiseSeamFinder>::delete() generated
	// ("cv::Ptr<cv::detail::PairwiseSeamFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_PairwiseSeamFinderG_delete(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::PairwiseSeamFinder>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::PairwiseSeamFinder>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_PairwiseSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::PairwiseSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

}

