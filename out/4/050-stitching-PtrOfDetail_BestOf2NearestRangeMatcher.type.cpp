extern "C" {
	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BestOf2NearestRangeMatcher* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestRangeMatcher* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::new_null() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_new_null_const() {
			return new cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>();
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::delete() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_delete(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::to_PtrOfDetail_BestOf2NearestMatcher() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::to_PtrOfDetail_BestOf2NearestMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_to_PtrOfDetail_BestOf2NearestMatcher(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(instance->dynamicCast<cv::detail::BestOf2NearestMatcher>());
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::to_PtrOfDetail_FeaturesMatcher() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::to_PtrOfDetail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}

	// cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>::new", vec![(pred!(const, ["val"], ["cv::detail::BestOf2NearestRangeMatcher"]), _)]),
	cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>* cv_PtrLcv_detail_BestOf2NearestRangeMatcherG_new_const_BestOf2NearestRangeMatcher(cv::detail::BestOf2NearestRangeMatcher* val) {
			return new cv::Ptr<cv::detail::BestOf2NearestRangeMatcher>(val);
	}

}

