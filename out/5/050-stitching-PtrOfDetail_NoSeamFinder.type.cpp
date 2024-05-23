extern "C" {
	// cv::Ptr<cv::detail::NoSeamFinder>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::NoSeamFinder* cv_PtrLcv_detail_NoSeamFinderG_getInnerPtr_const(const cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoSeamFinder>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoSeamFinder* cv_PtrLcv_detail_NoSeamFinderG_getInnerPtrMut(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoSeamFinder>::new_null() generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::NoSeamFinder>* cv_PtrLcv_detail_NoSeamFinderG_new_null_const() {
			return new cv::Ptr<cv::detail::NoSeamFinder>();
	}

	// cv::Ptr<cv::detail::NoSeamFinder>::delete() generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_NoSeamFinderG_delete(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::NoSeamFinder>::to_PtrOfDetail_SeamFinder() generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::to_PtrOfDetail_SeamFinder", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::SeamFinder>* cv_PtrLcv_detail_NoSeamFinderG_to_PtrOfDetail_SeamFinder(cv::Ptr<cv::detail::NoSeamFinder>* instance) {
			return new cv::Ptr<cv::detail::SeamFinder>(instance->dynamicCast<cv::detail::SeamFinder>());
	}

	// cv::Ptr<cv::detail::NoSeamFinder>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::NoSeamFinder>::new", vec![(pred!(const, ["val"], ["cv::detail::NoSeamFinder"]), _)]),
	cv::Ptr<cv::detail::NoSeamFinder>* cv_PtrLcv_detail_NoSeamFinderG_new_const_NoSeamFinder(cv::detail::NoSeamFinder* val) {
			return new cv::Ptr<cv::detail::NoSeamFinder>(val);
	}

}

