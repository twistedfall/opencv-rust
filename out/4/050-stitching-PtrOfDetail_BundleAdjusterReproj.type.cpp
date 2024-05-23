extern "C" {
	// cv::Ptr<cv::detail::BundleAdjusterReproj>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BundleAdjusterReproj* cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterReproj* cv_PtrLcv_detail_BundleAdjusterReprojG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::new_null() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterReproj>* cv_PtrLcv_detail_BundleAdjusterReprojG_new_null_const() {
			return new cv::Ptr<cv::detail::BundleAdjusterReproj>();
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::delete() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BundleAdjusterReprojG_delete(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::to_PtrOfDetail_BundleAdjusterBase() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::to_PtrOfDetail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterReprojG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterReproj>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BundleAdjusterReproj>::new", vec![(pred!(const, ["val"], ["cv::detail::BundleAdjusterReproj"]), _)]),
	cv::Ptr<cv::detail::BundleAdjusterReproj>* cv_PtrLcv_detail_BundleAdjusterReprojG_new_const_BundleAdjusterReproj(cv::detail::BundleAdjusterReproj* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterReproj>(val);
	}

}

