extern "C" {
	// cv::Ptr<cv::detail::BlocksGainCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BlocksGainCompensator* cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksGainCompensator* cv_PtrLcv_detail_BlocksGainCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BlocksGainCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::BlocksGainCompensator>();
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BlocksGainCompensatorG_delete(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::to_PtrOfDetail_BlocksCompensator() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::to_PtrOfDetail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BlocksCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_to_PtrOfDetail_BlocksCompensator(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return new cv::Ptr<cv::detail::BlocksCompensator>(instance->dynamicCast<cv::detail::BlocksCompensator>());
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksGainCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

	// cv::Ptr<cv::detail::BlocksGainCompensator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BlocksGainCompensator>::new", vec![(pred!(const, ["val"], ["cv::detail::BlocksGainCompensator"]), _)]),
	cv::Ptr<cv::detail::BlocksGainCompensator>* cv_PtrLcv_detail_BlocksGainCompensatorG_new_const_BlocksGainCompensator(cv::detail::BlocksGainCompensator* val) {
			return new cv::Ptr<cv::detail::BlocksGainCompensator>(val);
	}

}

