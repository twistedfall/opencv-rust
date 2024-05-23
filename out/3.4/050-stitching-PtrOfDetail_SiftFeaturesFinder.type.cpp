extern "C" {
	// cv::Ptr<cv::detail::SiftFeaturesFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::SiftFeaturesFinder* cv_PtrLcv_detail_SiftFeaturesFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SiftFeaturesFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::SiftFeaturesFinder* cv_PtrLcv_detail_SiftFeaturesFinderG_getInnerPtrMut(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SiftFeaturesFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::SiftFeaturesFinder>* cv_PtrLcv_detail_SiftFeaturesFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::SiftFeaturesFinder>();
	}

	// cv::Ptr<cv::detail::SiftFeaturesFinder>::delete() generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_SiftFeaturesFinderG_delete(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::SiftFeaturesFinder>::to_PtrOfDetail_FeaturesFinder() generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::to_PtrOfDetail_FeaturesFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrLcv_detail_SiftFeaturesFinderG_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SiftFeaturesFinder>* instance) {
			return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}

	// cv::Ptr<cv::detail::SiftFeaturesFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::SiftFeaturesFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::SiftFeaturesFinder"]), _)]),
	cv::Ptr<cv::detail::SiftFeaturesFinder>* cv_PtrLcv_detail_SiftFeaturesFinderG_new_const_SiftFeaturesFinder(cv::detail::SiftFeaturesFinder* val) {
			return new cv::Ptr<cv::detail::SiftFeaturesFinder>(val);
	}

}

