extern "C" {
	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::AffineBestOf2NearestMatcher* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::AffineBestOf2NearestMatcher* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::new_null() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_new_null_const() {
			return new cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>();
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::delete() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_delete(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::to_PtrOfDetail_BestOf2NearestMatcher() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::to_PtrOfDetail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_to_PtrOfDetail_BestOf2NearestMatcher(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(instance->dynamicCast<cv::detail::BestOf2NearestMatcher>());
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::to_PtrOfDetail_FeaturesMatcher() generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::to_PtrOfDetail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}

	// cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>::new", vec![(pred!(const, ["val"], ["cv::detail::AffineBestOf2NearestMatcher"]), _)]),
	cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>* cv_PtrLcv_detail_AffineBestOf2NearestMatcherG_new_const_AffineBestOf2NearestMatcher(cv::detail::AffineBestOf2NearestMatcher* val) {
			return new cv::Ptr<cv::detail::AffineBestOf2NearestMatcher>(val);
	}

}

