extern "C" {
	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::GraphCutSeamFinderGpu* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_getInnerPtr_const(const cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::GraphCutSeamFinderGpu* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_getInnerPtrMut(cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::new_null() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_new_null_const() {
			return new cv::Ptr<cv::detail::GraphCutSeamFinderGpu>();
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::delete() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_GraphCutSeamFinderGpuG_delete(cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_GraphCutSeamFinderBase() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_GraphCutSeamFinderBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinderBase>* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_GraphCutSeamFinderBase(cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinderBase>(instance->dynamicCast<cv::detail::GraphCutSeamFinderBase>());
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_PairwiseSeamFinder() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_PairwiseSeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::PairwiseSeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_PairwiseSeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			return new cv::Ptr<cv::detail::PairwiseSeamFinder>(instance->dynamicCast<cv::detail::PairwiseSeamFinder>());
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

	// cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::GraphCutSeamFinderGpu>::new", vec![(pred!(const, ["val"], ["cv::detail::GraphCutSeamFinderGpu"]), _)]),
	cv::Ptr<cv::detail::GraphCutSeamFinderGpu>* cv_PtrLcv_detail_GraphCutSeamFinderGpuG_new_const_GraphCutSeamFinderGpu(cv::detail::GraphCutSeamFinderGpu* val) {
			return new cv::Ptr<cv::detail::GraphCutSeamFinderGpu>(val);
	}

}

