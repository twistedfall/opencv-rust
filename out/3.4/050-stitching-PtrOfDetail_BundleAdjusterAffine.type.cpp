extern "C" {
	// cv::Ptr<cv::detail::BundleAdjusterAffine>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BundleAdjusterAffine* cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffine* cv_PtrLcv_detail_BundleAdjusterAffineG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::new_null() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterAffine>* cv_PtrLcv_detail_BundleAdjusterAffineG_new_null_const() {
			return new cv::Ptr<cv::detail::BundleAdjusterAffine>();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::delete() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BundleAdjusterAffineG_delete(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::to_PtrOfDetail_BundleAdjusterBase() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::to_PtrOfDetail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterAffineG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffine>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffine>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffine>::new", vec![(pred!(const, ["val"], ["cv::detail::BundleAdjusterAffine"]), _)]),
	cv::Ptr<cv::detail::BundleAdjusterAffine>* cv_PtrLcv_detail_BundleAdjusterAffineG_new_const_BundleAdjusterAffine(cv::detail::BundleAdjusterAffine* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterAffine>(val);
	}

}

