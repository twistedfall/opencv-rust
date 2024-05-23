extern "C" {
	// cv::Ptr<cv::CalibrateRobertson>::getInnerPtr() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::getInnerPtr", vec![(pred!(const, [], []), _)]),
	const cv::CalibrateRobertson* cv_PtrLcv_CalibrateRobertsonG_getInnerPtr_const(const cv::Ptr<cv::CalibrateRobertson>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateRobertson>::getInnerPtrMut() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::getInnerPtrMut", vec![(pred!(mut, [], []), _)]),
	cv::CalibrateRobertson* cv_PtrLcv_CalibrateRobertsonG_getInnerPtrMut(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return instance->get();
	}

	// cv::Ptr<cv::CalibrateRobertson>::new_null() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::new_null", vec![(pred!(const, [], []), _)]),
	cv::Ptr<cv::CalibrateRobertson>* cv_PtrLcv_CalibrateRobertsonG_new_null_const() {
			return new cv::Ptr<cv::CalibrateRobertson>();
	}

	// cv::Ptr<cv::CalibrateRobertson>::delete() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::delete", vec![(pred!(mut, [], []), _)]),
	void cv_PtrLcv_CalibrateRobertsonG_delete(cv::Ptr<cv::CalibrateRobertson>* instance) {
			delete instance;
	}

	// cv::Ptr<cv::CalibrateRobertson>::to_PtrOfAlgorithm() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::to_PtrOfAlgorithm", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::Algorithm>* cv_PtrLcv_CalibrateRobertsonG_to_PtrOfAlgorithm(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return new cv::Ptr<cv::Algorithm>(instance->dynamicCast<cv::Algorithm>());
	}

	// cv::Ptr<cv::CalibrateRobertson>::to_PtrOfCalibrateCRF() generated
	// ("cv::Ptr<cv::CalibrateRobertson>::to_PtrOfCalibrateCRF", vec![(pred!(mut, [], []), _)]),
	cv::Ptr<cv::CalibrateCRF>* cv_PtrLcv_CalibrateRobertsonG_to_PtrOfCalibrateCRF(cv::Ptr<cv::CalibrateRobertson>* instance) {
			return new cv::Ptr<cv::CalibrateCRF>(instance->dynamicCast<cv::CalibrateCRF>());
	}

}

