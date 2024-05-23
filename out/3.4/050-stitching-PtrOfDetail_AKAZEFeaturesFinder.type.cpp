extern "C" {
	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::AKAZEFeaturesFinder* cv_PtrLcv_detail_AKAZEFeaturesFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::AKAZEFeaturesFinder* cv_PtrLcv_detail_AKAZEFeaturesFinderG_getInnerPtrMut(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::AKAZEFeaturesFinder>* cv_PtrLcv_detail_AKAZEFeaturesFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::AKAZEFeaturesFinder>();
	}

	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::delete() generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_AKAZEFeaturesFinderG_delete(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::to_PtrOfDetail_FeaturesFinder() generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::to_PtrOfDetail_FeaturesFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrLcv_detail_AKAZEFeaturesFinderG_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::AKAZEFeaturesFinder>* instance) {
			return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}

	// cv::Ptr<cv::detail::AKAZEFeaturesFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::AKAZEFeaturesFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::AKAZEFeaturesFinder"]), _)]),
	cv::Ptr<cv::detail::AKAZEFeaturesFinder>* cv_PtrLcv_detail_AKAZEFeaturesFinderG_new_const_AKAZEFeaturesFinder(cv::detail::AKAZEFeaturesFinder* val) {
			return new cv::Ptr<cv::detail::AKAZEFeaturesFinder>(val);
	}

}

