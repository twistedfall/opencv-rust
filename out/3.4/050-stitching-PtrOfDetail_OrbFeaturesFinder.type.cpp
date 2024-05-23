extern "C" {
	// cv::Ptr<cv::detail::OrbFeaturesFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::OrbFeaturesFinder* cv_PtrLcv_detail_OrbFeaturesFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::OrbFeaturesFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::OrbFeaturesFinder* cv_PtrLcv_detail_OrbFeaturesFinderG_getInnerPtrMut(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::OrbFeaturesFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::OrbFeaturesFinder>* cv_PtrLcv_detail_OrbFeaturesFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::OrbFeaturesFinder>();
	}

	// cv::Ptr<cv::detail::OrbFeaturesFinder>::delete() generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_OrbFeaturesFinderG_delete(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::OrbFeaturesFinder>::to_PtrOfDetail_FeaturesFinder() generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::to_PtrOfDetail_FeaturesFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrLcv_detail_OrbFeaturesFinderG_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::OrbFeaturesFinder>* instance) {
			return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}

	// cv::Ptr<cv::detail::OrbFeaturesFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::OrbFeaturesFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::OrbFeaturesFinder"]), _)]),
	cv::Ptr<cv::detail::OrbFeaturesFinder>* cv_PtrLcv_detail_OrbFeaturesFinderG_new_const_OrbFeaturesFinder(cv::detail::OrbFeaturesFinder* val) {
			return new cv::Ptr<cv::detail::OrbFeaturesFinder>(val);
	}

}

