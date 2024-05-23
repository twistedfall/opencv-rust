extern "C" {
	// cv::Ptr<cv::detail::VoronoiSeamFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::VoronoiSeamFinder* cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::VoronoiSeamFinder* cv_PtrLcv_detail_VoronoiSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::VoronoiSeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::VoronoiSeamFinder>();
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::delete() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_VoronoiSeamFinderG_delete(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::to_PtrOfDetail_PairwiseSeamFinder() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::to_PtrOfDetail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::PairwiseSeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_PairwiseSeamFinder(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::PairwiseSeamFinder>(instance->dynamicCast<cv::detail::PairwiseSeamFinder>());
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::VoronoiSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

	// cv::Ptr<cv::detail::VoronoiSeamFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::VoronoiSeamFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::VoronoiSeamFinder"]), _)]),
	cv::Ptr<cv::detail::VoronoiSeamFinder>* cv_PtrLcv_detail_VoronoiSeamFinderG_new_const_VoronoiSeamFinder(cv::detail::VoronoiSeamFinder* val) {
			return new cv::Ptr<cv::detail::VoronoiSeamFinder>(val);
	}

}

