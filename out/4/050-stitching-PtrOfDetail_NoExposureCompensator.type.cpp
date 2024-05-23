extern "C" {
	// cv::Ptr<cv::detail::NoExposureCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::NoExposureCompensator* cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoExposureCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::NoExposureCompensator* cv_PtrLcv_detail_NoExposureCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::NoExposureCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::NoExposureCompensator>* cv_PtrLcv_detail_NoExposureCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::NoExposureCompensator>();
	}

	// cv::Ptr<cv::detail::NoExposureCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_NoExposureCompensatorG_delete(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::NoExposureCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_NoExposureCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::NoExposureCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

	// cv::Ptr<cv::detail::NoExposureCompensator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::NoExposureCompensator>::new", vec![(pred!(const, ["val"], ["cv::detail::NoExposureCompensator"]), _)]),
	cv::Ptr<cv::detail::NoExposureCompensator>* cv_PtrLcv_detail_NoExposureCompensatorG_new_const_NoExposureCompensator(cv::detail::NoExposureCompensator* val) {
			return new cv::Ptr<cv::detail::NoExposureCompensator>(val);
	}

}

