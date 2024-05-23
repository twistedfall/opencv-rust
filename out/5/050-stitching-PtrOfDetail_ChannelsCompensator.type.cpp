extern "C" {
	// cv::Ptr<cv::detail::ChannelsCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::ChannelsCompensator* cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::ChannelsCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::ChannelsCompensator* cv_PtrLcv_detail_ChannelsCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::ChannelsCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::ChannelsCompensator>* cv_PtrLcv_detail_ChannelsCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::ChannelsCompensator>();
	}

	// cv::Ptr<cv::detail::ChannelsCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_ChannelsCompensatorG_delete(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::ChannelsCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_ChannelsCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::ChannelsCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

	// cv::Ptr<cv::detail::ChannelsCompensator>::new(TraitClass) generated
	// ("cv::Ptr<cv::detail::ChannelsCompensator>::new", vec![(pred!(const, ["val"], ["cv::detail::ChannelsCompensator"]), _)]),
	cv::Ptr<cv::detail::ChannelsCompensator>* cv_PtrLcv_detail_ChannelsCompensatorG_new_const_ChannelsCompensator(cv::detail::ChannelsCompensator* val) {
			return new cv::Ptr<cv::detail::ChannelsCompensator>(val);
	}

}

