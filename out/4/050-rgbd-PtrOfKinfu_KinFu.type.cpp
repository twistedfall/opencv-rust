extern "C" {
	// cv::Ptr<cv::kinfu::KinFu>::getInnerPtr() generated
	// ("cv::Ptr<cv::kinfu::KinFu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::kinfu::KinFu* cv_PtrLcv_kinfu_KinFuG_getInnerPtr_const(const cv::Ptr<cv::kinfu::KinFu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::KinFu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::kinfu::KinFu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::kinfu::KinFu* cv_PtrLcv_kinfu_KinFuG_getInnerPtrMut(cv::Ptr<cv::kinfu::KinFu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::kinfu::KinFu>::new_null() generated
	// ("cv::Ptr<cv::kinfu::KinFu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::kinfu::KinFu>* cv_PtrLcv_kinfu_KinFuG_new_null_const() {
			return new cv::Ptr<cv::kinfu::KinFu>();
	}

	// cv::Ptr<cv::kinfu::KinFu>::delete() generated
	// ("cv::Ptr<cv::kinfu::KinFu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_kinfu_KinFuG_delete(cv::Ptr<cv::kinfu::KinFu>* instance) {
			delete instance;
	}

}

