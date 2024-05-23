extern "C" {
	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BestOf2NearestMatcher* cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtr_const(const cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BestOf2NearestMatcher* cv_PtrLcv_detail_BestOf2NearestMatcherG_getInnerPtrMut(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::new_null() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_BestOf2NearestMatcherG_new_null_const() {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>();
	}

	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::delete() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BestOf2NearestMatcherG_delete(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::to_PtrOfDetail_FeaturesMatcher() generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::to_PtrOfDetail_FeaturesMatcher", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesMatcher>* cv_PtrLcv_detail_BestOf2NearestMatcherG_to_PtrOfDetail_FeaturesMatcher(cv::Ptr<cv::detail::BestOf2NearestMatcher>* instance) {
			return new cv::Ptr<cv::detail::FeaturesMatcher>(instance->dynamicCast<cv::detail::FeaturesMatcher>());
	}

	// cv::Ptr<cv::detail::BestOf2NearestMatcher>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BestOf2NearestMatcher>::new", vec![(pred!(const, ["val"], ["cv::detail::BestOf2NearestMatcher"]), _)]),
	cv::Ptr<cv::detail::BestOf2NearestMatcher>* cv_PtrLcv_detail_BestOf2NearestMatcherG_new_const_BestOf2NearestMatcher(cv::detail::BestOf2NearestMatcher* val) {
			return new cv::Ptr<cv::detail::BestOf2NearestMatcher>(val);
	}

}

