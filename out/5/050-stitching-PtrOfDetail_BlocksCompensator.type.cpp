extern "C" {
	// cv::Ptr<cv::detail::BlocksCompensator>::getInnerPtr() generated
	// ("cv::Ptr<cv::detail::BlocksCompensator>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::detail::BlocksCompensator* cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtr_const(const cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksCompensator>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::detail::BlocksCompensator>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::detail::BlocksCompensator* cv_PtrLcv_detail_BlocksCompensatorG_getInnerPtrMut(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::detail::BlocksCompensator>::new_null() generated
	// ("cv::Ptr<cv::detail::BlocksCompensator>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::detail::BlocksCompensator>* cv_PtrLcv_detail_BlocksCompensatorG_new_null_const() {
			return new cv::Ptr<cv::detail::BlocksCompensator>();
	}

	// cv::Ptr<cv::detail::BlocksCompensator>::delete() generated
	// ("cv::Ptr<cv::detail::BlocksCompensator>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_detail_BlocksCompensatorG_delete(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::detail::BlocksCompensator>::to_PtrOfDetail_ExposureCompensator() generated
	// ("cv::Ptr<cv::detail::BlocksCompensator>::to_PtrOfDetail_ExposureCompensator", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::detail::ExposureCompensator>* cv_PtrLcv_detail_BlocksCompensatorG_to_PtrOfDetail_ExposureCompensator(cv::Ptr<cv::detail::BlocksCompensator>* instance) {
			return new cv::Ptr<cv::detail::ExposureCompensator>(instance->dynamicCast<cv::detail::ExposureCompensator>());
	}

}

