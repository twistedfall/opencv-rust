extern "C" {
	// cv::Ptr<cv::detail::SurfFeaturesFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::SurfFeaturesFinder* cv_PtrLcv_detail_SurfFeaturesFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::SurfFeaturesFinder* cv_PtrLcv_detail_SurfFeaturesFinderG_getInnerPtrMut(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::SurfFeaturesFinder>* cv_PtrLcv_detail_SurfFeaturesFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::SurfFeaturesFinder>();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinder>::delete() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_SurfFeaturesFinderG_delete(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinder>::to_PtrOfDetail_FeaturesFinder() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::to_PtrOfDetail_FeaturesFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrLcv_detail_SurfFeaturesFinderG_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SurfFeaturesFinder>* instance) {
			return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::SurfFeaturesFinder"]), _)]),
	cv::Ptr<cv::detail::SurfFeaturesFinder>* cv_PtrLcv_detail_SurfFeaturesFinderG_new_const_SurfFeaturesFinder(cv::detail::SurfFeaturesFinder* val) {
			return new cv::Ptr<cv::detail::SurfFeaturesFinder>(val);
	}

}

