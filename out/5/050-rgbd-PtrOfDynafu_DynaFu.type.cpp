extern "C" {
	// cv::Ptr<cv::dynafu::DynaFu>::getInnerPtr() generated
	// ("cv::Ptr<cv::dynafu::DynaFu>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::dynafu::DynaFu* cv_PtrLcv_dynafu_DynaFuG_getInnerPtr_const(const cv::Ptr<cv::dynafu::DynaFu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dynafu::DynaFu>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::dynafu::DynaFu>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::dynafu::DynaFu* cv_PtrLcv_dynafu_DynaFuG_getInnerPtrMut(cv::Ptr<cv::dynafu::DynaFu>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::dynafu::DynaFu>::new_null() generated
	// ("cv::Ptr<cv::dynafu::DynaFu>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::dynafu::DynaFu>* cv_PtrLcv_dynafu_DynaFuG_new_null_const() {
			return new cv::Ptr<cv::dynafu::DynaFu>();
	}

	// cv::Ptr<cv::dynafu::DynaFu>::delete() generated
	// ("cv::Ptr<cv::dynafu::DynaFu>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_dynafu_DynaFuG_delete(cv::Ptr<cv::dynafu::DynaFu>* instance) {
			delete instance;
	}

}

