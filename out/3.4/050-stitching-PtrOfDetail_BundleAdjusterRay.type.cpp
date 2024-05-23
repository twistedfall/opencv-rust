extern "C" {
	// cv::Ptr<cv::detail::BundleAdjusterRay>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BundleAdjusterRay* cv_PtrLcv_detail_BundleAdjusterRayG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterRay* cv_PtrLcv_detail_BundleAdjusterRayG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::new_null() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterRay>* cv_PtrLcv_detail_BundleAdjusterRayG_new_null_const() {
			return new cv::Ptr<cv::detail::BundleAdjusterRay>();
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::delete() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BundleAdjusterRayG_delete(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::to_PtrOfDetail_BundleAdjusterBase() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::to_PtrOfDetail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterRayG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterRayG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterRay>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BundleAdjusterRay>::new", vec![(pred!(const, ["val"], ["cv::detail::BundleAdjusterRay"]), _)]),
	cv::Ptr<cv::detail::BundleAdjusterRay>* cv_PtrLcv_detail_BundleAdjusterRayG_new_const_BundleAdjusterRay(cv::detail::BundleAdjusterRay* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterRay>(val);
	}

}

