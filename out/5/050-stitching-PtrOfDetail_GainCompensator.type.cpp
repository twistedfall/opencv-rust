extern "C" {
	// cv::Ptr<cv::detail::GainCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::GainCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::GainCompensator* cv_PtrLcv_detail_GainCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::GainCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GainCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::GainCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::GainCompensator* cv_PtrLcv_detail_GainCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::GainCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::GainCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::GainCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::GainCompensator>* cv_PtrLcv_detail_GainCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::GainCompensator>();
	}

	// cv::Ptr<cv::detail::GainCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::GainCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_GainCompensatorG_delete(cv::Ptr<cv::detail::GainCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::GainCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::GainCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_GainCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::GainCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

	// cv::Ptr<cv::detail::GainCompensator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::GainCompensator>::new", vec![(pred!(const, ["val"], ["cv::detail::GainCompensator"]), _)]),
	cv::Ptr<cv::detail::GainCompensator>* cv_PtrLcv_detail_GainCompensatorG_new_const_GainCompensator(cv::detail::GainCompensator* val) {
			return new cv::Ptr<cv::detail::GainCompensator>(val);
	}

}

