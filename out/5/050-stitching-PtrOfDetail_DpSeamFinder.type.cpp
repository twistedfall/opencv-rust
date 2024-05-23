extern "C" {
	// cv::Ptr<cv::detail::DpSeamFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::DpSeamFinder* cv_PtrLcv_detail_DpSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::DpSeamFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::DpSeamFinder* cv_PtrLcv_detail_DpSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::DpSeamFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::DpSeamFinder>* cv_PtrLcv_detail_DpSeamFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::DpSeamFinder>();
	}

	// cv::Ptr<cv::detail::DpSeamFinder>::delete() generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_DpSeamFinderG_delete(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::DpSeamFinder>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_DpSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::DpSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

	// cv::Ptr<cv::detail::DpSeamFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::DpSeamFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::DpSeamFinder"]), _)]),
	cv::Ptr<cv::detail::DpSeamFinder>* cv_PtrLcv_detail_DpSeamFinderG_new_const_DpSeamFinder(cv::detail::DpSeamFinder* val) {
			return new cv::Ptr<cv::detail::DpSeamFinder>(val);
	}

}

