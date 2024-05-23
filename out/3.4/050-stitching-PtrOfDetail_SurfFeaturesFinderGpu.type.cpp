extern "C" {
	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::SurfFeaturesFinderGpu* cv_PtrLcv_detail_SurfFeaturesFinderGpuG_getInnerPtr_const(const cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::SurfFeaturesFinderGpu* cv_PtrLcv_detail_SurfFeaturesFinderGpuG_getInnerPtrMut(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::new_null() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* cv_PtrLcv_detail_SurfFeaturesFinderGpuG_new_null_const() {
			return new cv::Ptr<cv::detail::SurfFeaturesFinderGpu>();
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::delete() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_SurfFeaturesFinderGpuG_delete(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::to_PtrOfDetail_FeaturesFinder() generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::to_PtrOfDetail_FeaturesFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::FeaturesFinder>* cv_PtrLcv_detail_SurfFeaturesFinderGpuG_to_PtrOfDetail_FeaturesFinder(cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* instance) {
			return new cv::Ptr<cv::detail::FeaturesFinder>(instance->dynamicCast<cv::detail::FeaturesFinder>());
	}

	// cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::SurfFeaturesFinderGpu>::new", vec![(pred!(const, ["val"], ["cv::detail::SurfFeaturesFinderGpu"]), _)]),
	cv::Ptr<cv::detail::SurfFeaturesFinderGpu>* cv_PtrLcv_detail_SurfFeaturesFinderGpuG_new_const_SurfFeaturesFinderGpu(cv::detail::SurfFeaturesFinderGpu* val) {
			return new cv::Ptr<cv::detail::SurfFeaturesFinderGpu>(val);
	}

}

