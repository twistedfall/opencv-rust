extern "C" {
	// cv::Ptr<cv::sfm::BaseSFM>::getInnerPtr() generated
	// ("cv::Ptr<cv::sfm::BaseSFM>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::sfm::BaseSFM* cv_PtrLcv_sfm_BaseSFMG_getInnerPtr_const(const cv::Ptr<cv::sfm::BaseSFM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::sfm::BaseSFM>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::sfm::BaseSFM>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::sfm::BaseSFM* cv_PtrLcv_sfm_BaseSFMG_getInnerPtrMut(cv::Ptr<cv::sfm::BaseSFM>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::sfm::BaseSFM>::new_null() generated
	// ("cv::Ptr<cv::sfm::BaseSFM>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::sfm::BaseSFM>* cv_PtrLcv_sfm_BaseSFMG_new_null_const() {
			return new cv::Ptr<cv::sfm::BaseSFM>();
	}

	// cv::Ptr<cv::sfm::BaseSFM>::delete() generated
	// ("cv::Ptr<cv::sfm::BaseSFM>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_sfm_BaseSFMG_delete(cv::Ptr<cv::sfm::BaseSFM>* instance) {
			delete instance;
	}

}

