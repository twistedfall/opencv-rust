extern "C" {
	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BundleAdjusterAffinePartial* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtr_const(const cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BundleAdjusterAffinePartial* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_getInnerPtrMut(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::new_null() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_new_null_const() {
			return new cv::Ptr<cv::detail::BundleAdjusterAffinePartial>();
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::delete() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BundleAdjusterAffinePartialG_delete(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::to_PtrOfDetail_BundleAdjusterBase() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::to_PtrOfDetail_BundleAdjusterBase", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::to_PtrOfDetail_Estimator() generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::to_PtrOfDetail_Estimator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::Estimator>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* instance) {
			return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}

	// cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BundleAdjusterAffinePartial>::new", vec![(pred!(const, ["val"], ["cv::detail::BundleAdjusterAffinePartial"]), _)]),
	cv::Ptr<cv::detail::BundleAdjusterAffinePartial>* cv_PtrLcv_detail_BundleAdjusterAffinePartialG_new_const_BundleAdjusterAffinePartial(cv::detail::BundleAdjusterAffinePartial* val) {
			return new cv::Ptr<cv::detail::BundleAdjusterAffinePartial>(val);
	}

}

