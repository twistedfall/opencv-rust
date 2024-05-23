extern "C" {
	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BlocksChannelsCompensator* cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksChannelsCompensator* cv_PtrLcv_detail_BlocksChannelsCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BlocksChannelsCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::BlocksChannelsCompensator>();
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BlocksChannelsCompensatorG_delete(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::to_PtrOfDetail_BlocksCompensator() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::to_PtrOfDetail_BlocksCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::BlocksCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_BlocksCompensator(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::BlocksCompensator>(instance->dynamicCast<cv::detail::BlocksCompensator>());
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

	// cv::Ptr<cv::detail::BlocksChannelsCompensator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::BlocksChannelsCompensator>::new", vec![(pred!(const, ["val"], ["cv::detail::BlocksChannelsCompensator"]), _)]),
	cv::Ptr<cv::detail::BlocksChannelsCompensator>* cv_PtrLcv_detail_BlocksChannelsCompensatorG_new_const_BlocksChannelsCompensator(cv::detail::BlocksChannelsCompensator* val) {
			return new cv::Ptr<cv::detail::BlocksChannelsCompensator>(val);
	}

}

