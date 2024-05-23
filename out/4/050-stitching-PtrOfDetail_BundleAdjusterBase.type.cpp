extern "C" {
	// cv::Ptr<cv::detail::BundleAdjusterBase>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterBase>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BundleAdjusterBase* cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterBase>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterBase>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterBase* cv_PtrLcv_detail_BundleAdjusterBaseG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterBase>::new_null() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterBase>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterBaseG_new_null_const() {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>();
	}

	// cv::Ptr<cv::detail::BundleAdjusterBase>::delete() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterBase>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BundleAdjusterBaseG_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BundleAdjusterBase>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterBase>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterBaseG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

}

