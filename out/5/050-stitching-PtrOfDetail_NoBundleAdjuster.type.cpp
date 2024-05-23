extern "C" {
	// cv::Ptr<cv::detail::NoBundleAdjuster>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::NoBundleAdjuster* cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtr_const(const cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoBundleAdjuster* cv_PtrLcv_detail_NoBundleAdjusterG_getInnerPtrMut(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::new_null() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::NoBundleAdjuster>* cv_PtrLcv_detail_NoBundleAdjusterG_new_null_const() {
			return new cv::Ptr<cv::detail::NoBundleAdjuster>();
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::delete() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_NoBundleAdjusterG_delete(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::to_PtrOfDetail_BundleAdjusterBase() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::to_PtrOfDetail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_NoBundleAdjusterG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::NoBundleAdjuster>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::NoBundleAdjuster>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::NoBundleAdjuster>::new", vec![(pred!(const, ["val"], ["cv::detail::NoBundleAdjuster"]), _)]),
	cv::Ptr<cv::detail::NoBundleAdjuster>* cv_PtrLcv_detail_NoBundleAdjusterG_new_const_NoBundleAdjuster(cv::detail::NoBundleAdjuster* val) {
			return new cv::Ptr<cv::detail::NoBundleAdjuster>(val);
	}

}

